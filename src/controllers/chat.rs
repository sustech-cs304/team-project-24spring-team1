use actix_web::{get, web, Responder};
use diesel_async::AsyncConnection;
use scoped_futures::ScopedFutureExt;
use serde::Serialize;

use super::auth::JwtAuth;
use super::AppState;
use crate::error::{Error, Result};
use crate::orm::chat::member::{ChatMember, NewChatMember};
use crate::orm::chat::NewChat;
use crate::orm::schema::{chat_members, chats};
use crate::orm::utils::RunQueryDsl;

// ===== Handlers =====

#[derive(Debug, Serialize)]
struct ChatIdResponse {
    pub chat_id: i32,
}

#[get("/with/{id}")]
async fn get_chat_id_with(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let members = [auth.account_id, path.into_inner()];
    if members[0] == members[1] {
        return Err(Error::NotAcceptable("Cannot chat with yourself".into()));
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

                let chat_id = NewChat { is_group: false }
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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_chat_id_with);
}
