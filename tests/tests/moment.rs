use actix_web::http::StatusCode;
use actix_web::test;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::common::{create_app, TestApp};
use crate::tests::account::AccountCard;
use crate::tests::auth::{create_default_account, AccountInfo};
use crate::tests::misc::Page;

#[derive(Debug, Serialize)]
pub struct NewMomentForm<'a> {
    pub content: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MomentIdResponse {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MomentResponse {
    pub id: i32,
    pub account: AccountCard,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ListMomentsResponse {
    pub page: Page,
    pub moments: Vec<MomentResponse>,
}

#[derive(Debug, Serialize)]
struct NewCommentForm<'a> {
    pub content: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommentDisplay {
    pub id: i32,
    pub account: AccountCard,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CommentsResponse {
    pub page: Page,
    pub comments: Vec<CommentDisplay>,
}

pub async fn create_moment(
    app: impl TestApp,
    account: &AccountInfo,
    form: &NewMomentForm<'_>,
) -> i32 {
    let req = test::TestRequest::post()
        .uri("/api/moment")
        .insert_header(account.to_header_pair())
        .set_json(form)
        .to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let id: MomentIdResponse = test::read_body_json(resp).await;
    id.id
}

pub async fn get_moment(app: impl TestApp, moment_id: i32) -> MomentResponse {
    let req = test::TestRequest::get()
        .uri(&format!("/api/moment/{moment_id}"))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    test::read_body_json(resp).await
}

lazy_static! {
    static ref DEFAULT_MOMENT: NewMomentForm<'static> = NewMomentForm {
        content: "Test Moment 1",
    };
}

pub async fn create_default_moment(app: impl TestApp, account: &AccountInfo) -> i32 {
    create_moment(app, account, &DEFAULT_MOMENT).await
}

#[actix_web::test]
async fn test_moment_create_and_get() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let moment_id = create_default_moment(&app, &account).await;

    let moment = get_moment(&app, moment_id).await;
    assert_eq!(moment.id, moment_id);
    assert_eq!(moment.content, DEFAULT_MOMENT.content);
    assert_eq!(moment.account.id, account.account_id);
}

#[actix_web::test]
async fn test_moment_list() {
    let app = create_app().await;

    let account = create_default_account(&app).await;

    let moment_2 = NewMomentForm {
        content: "Test Moment 2",
    };

    let moment_id_1 = create_moment(&app, &account, &DEFAULT_MOMENT).await;
    let moment_id_2 = create_moment(&app, &account, &moment_2).await;

    let req = test::TestRequest::get().uri("/api/moment").to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let moments: ListMomentsResponse = test::read_body_json(resp).await;
    assert_eq!(moments.moments.len(), 2);
    assert_eq!(moments.page.total_item, 2);

    let moment = moments
        .moments
        .iter()
        .find(|e| e.id == moment_id_1)
        .unwrap();
    assert_eq!(moment.content, DEFAULT_MOMENT.content);
    let moment = moments
        .moments
        .iter()
        .find(|e| e.id == moment_id_2)
        .unwrap();
    assert_eq!(moment.content, moment.content);
}

#[actix_web::test]
async fn test_moment_delete() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let moment_id = create_default_moment(&app, &account).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/moment/{moment_id}"))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/api/moment/{moment_id}"))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_moment_comment() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let moment_id = create_default_moment(&app, &account).await;

    let form = NewCommentForm { content: "Hello" };

    let req = test::TestRequest::post()
        .uri(&format!("/api/moment/{moment_id}/comment"))
        .insert_header(account.to_header_pair())
        .set_json(&form)
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/api/moment/{moment_id}/comment"))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let comments: CommentsResponse = test::read_body_json(resp).await;
    assert_eq!(comments.comments.len(), 1);
    assert_eq!(comments.comments[0].content, form.content);
}
