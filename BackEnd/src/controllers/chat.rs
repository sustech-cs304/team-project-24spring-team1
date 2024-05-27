use actix_web::{get, post, web, Responder};
use diesel::prelude::*;
use diesel_async::AsyncConnection;
use scoped_futures::ScopedFutureExt;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

use super::auth::JwtAuth;
use super::AppState;
use crate::error::{Error, Result};
use crate::orm::chat::member::{ChatItemDisplay, ChatMember, ChatMemberDisplay, NewChatMember};
use crate::orm::chat::message::{ChatMessage, ChatMessageDisplay, NewChatMessage};
use crate::orm::chat::NewChat;
use crate::orm::schema::{chat_members, chat_messages, chats};
use crate::orm::utils::RunQueryDsl;
use crate::orm::Conn;
use crate::utils::page::{Page, PaginateQuery};

// ===== Handlers =====

#[derive(Debug, Serialize)]
struct ChatListResponse {
    pub page: Page,
    pub chats: Vec<ChatItemDisplay>,
}

#[derive(Debug, Serialize)]
struct ChatMemberResponse {
    pub page: Page,
    pub members: Vec<ChatMemberDisplay>,
}

#[derive(Debug, Deserialize, Validate)]
struct ChatIdQuery {
    pub with: i32,
}

#[derive(Debug, Serialize)]
struct ChatIdResponse {
    pub chat_id: i32,
}

#[derive(Debug, Serialize)]
struct ChatMessageResponse {
    pub page: Page,
    pub messages: Vec<ChatMessageDisplay>,
}

#[derive(Debug, Deserialize, Validate)]
struct NewChatMessageForm {
    pub content: String,
}

#[get("")]
async fn get_chat_list(
    state: web::Data<AppState>,
    page: web::Query<PaginateQuery>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let total_item = ChatMember::by_account_id(auth.account_id)
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;
    let page = Page::builder(total_item, page.page.unwrap_or(1)).build();

    let chats = ChatMember::by_account_id_display(auth.account_id)
        .limit(page.page_size)
        .offset(page.offset)
        .get_results(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(ChatListResponse { page, chats }))
}

#[get("/get_id")]
async fn get_chat_id_with(
    state: web::Data<AppState>,
    query: web::Query<ChatIdQuery>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let members = [auth.account_id, query.with];
    if members[0] == members[1] {
        let mut errors = ValidationErrors::new();
        errors.add("with", ValidationError::new("same_account"));
        return Err(Error::InvalidArgument(errors));
    }

    let mut conn = state.pool.get().await?;
    let chat_id = conn
        .transaction::<_, Error, _>(|conn| {
            async move {
                // If the chat already exists, return it
                match ChatMember::by_friend_chat(&members).first(conn).await {
                    Err(diesel::result::Error::NotFound) => {}
                    result => return Ok(result?),
                }

                let chat_id = NewChat {
                    is_group: false,
                    name: "Friend Chat",
                }
                .as_insert()
                .returning(chats::id)
                .get_result(conn)
                .await?;

                let members = members.map(|account_id| NewChatMember::new(chat_id, account_id));
                diesel::insert_into(chat_members::table)
                    .values(&members)
                    .execute(conn)
                    .await?;

                Ok(chat_id)
            }
            .scope_boxed()
        })
        .await?;

    Ok(web::Json(ChatIdResponse { chat_id }))
}

#[get("/{id}/member")]
async fn get_chat_members(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    query: web::Query<PaginateQuery>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let chat_id = path.into_inner();
    check_in_chat(auth.account_id, chat_id, &mut state.pool.get().await?).await?;

    let total_item = ChatMember::by_chat_id(chat_id)
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;
    let page = Page::builder(total_item, query.page.unwrap_or(1)).build();

    let members = ChatMember::by_chat_id_display(chat_id)
        .order(chat_members::account_id.desc())
        .limit(page.page_size)
        .offset(page.offset)
        .get_results(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(ChatMemberResponse { page, members }))
}

#[get("/{id}/message")]
async fn get_chat_messages(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    query: web::Query<PaginateQuery>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let chat_id = path.into_inner();
    check_in_chat(auth.account_id, chat_id, &mut state.pool.get().await?).await?;

    let total_item = ChatMessage::by_chat_id(chat_id)
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;
    let page = Page::builder(total_item, query.page.unwrap_or(1)).build();

    let messages = ChatMessage::by_chat_id_display(chat_id)
        .order(chat_messages::created_at.desc())
        .limit(page.page_size)
        .offset(page.offset)
        .get_results(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(ChatMessageResponse { page, messages }))
}

#[post("/{id}/message")]
async fn post_chat_messages(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<NewChatMessageForm>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let chat_id = path.into_inner();
    let form = form.into_inner();
    form.validate()?;
    check_in_chat(auth.account_id, chat_id, &mut state.pool.get().await?).await?;

    let new_message = NewChatMessage {
        chat_id,
        account_id: auth.account_id,
        content: &form.content,
    };
    new_message
        .as_insert()
        .execute(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chat_list)
        .service(get_chat_id_with)
        .service(get_chat_members)
        .service(get_chat_messages)
        .service(post_chat_messages);
}

// ===== Other Functions =====

async fn check_in_chat(account_id: i32, chat_id: i32, conn: &mut Conn) -> Result<()> {
    let count: i64 = ChatMember::check_in_chat(account_id, chat_id)
        .first(conn)
        .await?;
    if count == 0 {
        return Err(Error::Unauthorized("Not in chat".to_owned()));
    }

    Ok(())
}
