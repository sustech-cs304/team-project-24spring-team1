use actix_web::http::StatusCode;
use actix_web::test;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::{create_app, TestApp};
use crate::tests::auth::{
    create_account, create_default_account, RegisterForm, TEST_DEFAULT_ACCOUNT_FORM,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    Staff,
    Student,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountProfile {
    pub id: i32,
    pub sustech_id: i32,
    pub name: String,
    pub email: String,
    pub avatar: Uuid,
    pub role: Role,
    pub bio: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountCard {
    pub id: i32,
    pub name: String,
    pub avatar: Uuid,
    pub role: Role,
}

#[derive(Debug, Default, Serialize)]
struct UpdateProfileForm<'a> {
    pub email: Option<&'a str>,
    pub bio: Option<&'a str>,
    pub avatar: Option<Uuid>,
}

pub async fn get_profile(app: impl TestApp, account_id: i32) -> AccountProfile {
    let req = test::TestRequest::get()
        .uri(&format!("/api/account/{}/profile", account_id))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let profile = test::read_body_json(resp).await;
    log::trace!("Profile response: {profile:?}");
    profile
}

#[tokio::test]
async fn test_profile() {
    let app = create_app().await;
    let account = create_default_account(&app).await;

    let profile: AccountProfile = get_profile(&app, account.account_id).await;
    assert_eq!(profile.id, account.account_id);
    assert_eq!(profile.sustech_id, TEST_DEFAULT_ACCOUNT_FORM.sustech_id);
    assert_eq!(profile.name, TEST_DEFAULT_ACCOUNT_FORM.name);
}

#[tokio::test]
async fn test_profile_update() {
    let app = create_app().await;
    let account = create_default_account(&app).await;

    const NEW_BIO: &str = "Hello, world!";

    let req = test::TestRequest::put()
        .uri(&format!("/api/account/{}/profile", account.account_id))
        .append_header(account.to_header_pair())
        .set_json(UpdateProfileForm {
            bio: Some(NEW_BIO),
            ..Default::default()
        })
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let profile: AccountProfile = get_profile(&app, account.account_id).await;
    assert_eq!(profile.bio, NEW_BIO);
}

#[tokio::test]
async fn test_profile_no_update_other() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let another = create_account(
        &app,
        &RegisterForm {
            sustech_id: 12345678,
            name: "another",
            password: "password",
        },
    )
    .await;

    let old_profile: AccountProfile = get_profile(&app, account.account_id).await;

    let req = test::TestRequest::put()
        .uri(&format!("/api/account/{}/profile", account.account_id))
        .append_header(another.to_header_pair())
        .set_json(UpdateProfileForm {
            bio: Some("Hello, world!"),
            ..Default::default()
        })
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let profile: AccountProfile = get_profile(&app, account.account_id).await;
    assert_eq!(profile.bio, old_profile.bio);
}
