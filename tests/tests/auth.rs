use actix_web::http::StatusCode;
use actix_web::test;
use serde::{Deserialize, Serialize};

use crate::common::{create_app, TestApp};

#[derive(Debug, Serialize)]
pub struct RegisterForm<'a> {
    pub sustech_id: i32,
    pub name: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize)]
pub struct LoginForm<'a> {
    pub sustech_id: i32,
    pub password: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountInfo {
    pub account_id: i32,
    pub token: String,
}

pub const TEST_DEFAULT_ACCOUNT_FORM: RegisterForm<'_> = RegisterForm {
    sustech_id: 11111111,
    name: "test",
    password: "password",
};

pub async fn create_account(app: impl TestApp, form: &RegisterForm<'_>) -> AccountInfo {
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(form)
        .to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let info = test::read_body_json(resp).await;
    log::trace!("Register response: {info:?}");

    info
}

pub async fn create_default_account(app: impl TestApp) -> AccountInfo {
    create_account(app, &TEST_DEFAULT_ACCOUNT_FORM).await
}

#[tokio::test]
async fn test_register() {
    let app = create_app().await;

    let _account = create_default_account(&app).await;
}

#[tokio::test]
async fn test_login() {
    let app = create_app().await;
    let account = create_default_account(&app).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&LoginForm {
            sustech_id: TEST_DEFAULT_ACCOUNT_FORM.sustech_id,
            password: TEST_DEFAULT_ACCOUNT_FORM.password,
        })
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let info: AccountInfo = test::read_body_json(resp).await;
    assert_eq!(info.account_id, account.account_id);
}

#[tokio::test]
async fn test_token() {
    let app = create_app().await;
    let account = create_default_account(&app).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/check")
        .append_header(account.to_header_pair())
        .to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_token_invalid_schema() {
    let app = create_app().await;
    let account = create_default_account(&app).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/check")
        .append_header(("Authorization", format!("Basic {}", account.token)))
        .to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_token_invalid_data() {
    let app = create_app().await;

    let req = test::TestRequest::get()
        .uri("/api/auth/check")
        .append_header((
            "Authorization",
            "Basic eyJhbGciOiJIUzI1NiJ9.e30.4E_Bsx-pJi3kOW9wVXN8CgbATwP09D9V5gxh9-9zSZ0",
        ))
        .to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ===== Other Functions =====

impl AccountInfo {
    pub fn to_header_pair(&self) -> (&str, String) {
        ("Authorization", format!("Bearer {}", self.token))
    }
}
