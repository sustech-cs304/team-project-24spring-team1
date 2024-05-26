use actix_web::http::StatusCode;
use actix_web::test;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::common::{create_app, TestApp};
use crate::tests::account::AccountCard;
use crate::tests::auth::{create_account, create_default_account, AccountInfo, RegisterForm};
use crate::tests::misc::Page;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChatIdResponse {
    pub chat_id: i32,
}

#[derive(Debug, Serialize)]
struct NewMessageForm<'a> {
    pub content: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChatMessage {
    pub id: i32,
    pub account_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChatResponse {
    pub id: i32,
    pub name: String,
    pub is_group: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChatMessagesResponse {
    pub page: Page,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChatMembersResponse {
    pub page: Page,
    pub members: Vec<AccountCard>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChatsResponse {
    pub page: Page,
    pub chats: Vec<ChatResponse>,
}

async fn create_account_pair(app: impl TestApp) -> (AccountInfo, AccountInfo) {
    let account_1 = create_default_account(&app).await;
    let account_2 = create_account(
        &app,
        &RegisterForm {
            sustech_id: 12345678,
            name: "User2",
            password: "password",
        },
    )
    .await;
    (account_1, account_2)
}

async fn get_chat(
    app: impl TestApp,
    primary_account: &AccountInfo,
    secondary_account: &AccountInfo,
) -> i32 {
    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/chat/get_id?with={}",
            secondary_account.account_id
        ))
        .insert_header(primary_account.to_header_pair())
        .to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let id: ChatIdResponse = test::read_body_json(resp).await;
    id.chat_id
}

#[actix_web::test]
async fn test_chat_commutative() {
    let app = create_app().await;
    let (account_1, account_2) = create_account_pair(&app).await;

    let id_lhs = get_chat(&app, &account_1, &account_2).await;
    let id_rhs = get_chat(&app, &account_2, &account_1).await;
    assert_eq!(id_lhs, id_rhs);
}

#[actix_web::test]
async fn test_chat_no_reflexive() {
    let app = create_app().await;
    let account = create_default_account(&app).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/chat/get_id?with={}", account.account_id))
        .insert_header(account.to_header_pair())
        .to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_chat_message() {
    let app = create_app().await;
    let (account_1, account_2) = create_account_pair(&app).await;
    let chat_id = get_chat(&app, &account_1, &account_2).await;

    let form = NewMessageForm {
        content: "Message 1",
    };
    let req = test::TestRequest::post()
        .uri(&format!("/api/chat/{chat_id}/message"))
        .insert_header(account_1.to_header_pair())
        .set_json(&form)
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/api/chat/{chat_id}/message"))
        .insert_header(account_2.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let messages: ChatMessagesResponse = test::read_body_json(resp).await;
    assert_eq!(messages.messages.len(), 1);
    let message = &messages.messages[0];
    assert_eq!(message.account_id, account_1.account_id);
    assert_eq!(message.content, form.content);
}

#[actix_web::test]
async fn test_chat_member() {
    let app = create_app().await;
    let (account_1, account_2) = create_account_pair(&app).await;
    let chat_id = get_chat(&app, &account_1, &account_2).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/chat/{chat_id}/member"))
        .insert_header(account_1.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let messages: ChatMembersResponse = test::read_body_json(resp).await;
    let members_got = messages
        .members
        .iter()
        .map(|account| account.id)
        .collect::<HashSet<_>>();
    let members_expected = [&account_1, &account_2]
        .iter()
        .map(|account| account.account_id)
        .collect::<HashSet<_>>();
    assert_eq!(members_got, members_expected);
}

#[actix_web::test]
async fn test_chat_list() {
    let app = create_app().await;
    let (account_1, account_2) = create_account_pair(&app).await;
    let chat_id = get_chat(&app, &account_1, &account_2).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/chat"))
        .insert_header(account_1.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let chats: ChatsResponse = test::read_body_json(resp).await;
    assert_eq!(chats.chats.len(), 1);
    assert_eq!(chats.chats[0].id, chat_id);
}
