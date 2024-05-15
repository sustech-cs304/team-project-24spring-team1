use actix_web::http::StatusCode;
use actix_web::test;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::common::{create_app, TestApp};
use crate::tests::account::AccountCard;
use crate::tests::auth::{create_account, create_default_account, AccountInfo, RegisterForm};
use crate::tests::misc::Page;
use crate::tests::misc::Place;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Show,
    Lecture,
    Competition,
    Other,
}

#[derive(Debug, Serialize)]
pub struct NewEventForm<'a> {
    pub name: &'a str,
    pub kind: EventType,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: i32,
    pub description: &'a str,
    pub tickets: Option<i32>,
    pub registeration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventStatus {
    Applicable,
    NotStarted,
    Ongoing,
    Ended,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EventIdResponse {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventDisplayResponse {
    pub id: i32,
    pub name: String,
    pub kind: EventType,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue: Place,
    pub description: String,
    pub organizer: AccountCard,
    pub tickets: Option<i32>,
    pub registeration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventSummaryResponse {
    pub id: i32,
    pub name: String,
    pub kind: EventType,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue: Place,
    pub description: String,
    pub organizer: AccountCard,
    pub tickets: Option<i32>,
    pub registeration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventWithParticipation<T> {
    pub participation_count: i64,
    #[serde(flatten)]
    pub inner: T,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ListEventsResponse {
    pub page: Page,
    pub event_by_id: Option<EventWithParticipation<EventSummaryResponse>>,
    pub events: Vec<EventWithParticipation<EventSummaryResponse>>,
}

fn parse_datetime(date: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap()
}

pub async fn create_event(
    app: impl TestApp,
    account: &AccountInfo,
    form: &NewEventForm<'_>,
) -> i32 {
    let req = test::TestRequest::post()
        .uri("/api/event")
        .insert_header(account.to_header_pair())
        .set_json(form)
        .to_request();

    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let id: EventIdResponse = test::read_body_json(resp).await;
    id.id
}

lazy_static! {
    static ref DEFAULT_EVENT: NewEventForm<'static> = NewEventForm {
        name: "Test Event",
        kind: EventType::Show,
        start_at: parse_datetime("2021-01-01 00:00:00"),
        end_at: parse_datetime("2021-01-02 00:00:00"),
        venue_id: 1,
        description: "Test Description",
        tickets: None,
        registeration_deadline: None,
    };
}

pub async fn create_default_event(app: impl TestApp, account: &AccountInfo) -> i32 {
    create_event(app, account, &DEFAULT_EVENT).await
}

#[actix_web::test]
async fn test_event_create_and_get() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let event_id = create_default_event(&app, &account).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/event/{}", event_id))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let event: EventWithParticipation<EventDisplayResponse> = test::read_body_json(resp).await;
    assert_eq!(event.inner.id, event_id);
    assert_eq!(event.inner.name, DEFAULT_EVENT.name);
    assert_eq!(event.inner.kind, DEFAULT_EVENT.kind);
    assert_eq!(event.inner.start_at, DEFAULT_EVENT.start_at);
    assert_eq!(event.inner.end_at, DEFAULT_EVENT.end_at);
    assert_eq!(event.inner.venue.id, DEFAULT_EVENT.venue_id);
    assert_eq!(event.inner.description, DEFAULT_EVENT.description);
    assert_eq!(event.inner.organizer.id, account.account_id);
}

#[actix_web::test]
async fn test_event_list() {
    let app = create_app().await;

    const EVENT_NAME_1: &str = "Test Event 1";
    const EVENT_NAME_2: &str = "Test Event 2";

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

    let event_id_1 = create_event(
        &app,
        &account_1,
        &NewEventForm {
            name: EVENT_NAME_1,
            kind: EventType::Show,
            start_at: parse_datetime("2021-01-01 00:00:00"),
            end_at: parse_datetime("2021-01-02 00:00:00"),
            venue_id: 1,
            description: "Test Description 1",
            tickets: None,
            registeration_deadline: None,
        },
    )
    .await;
    let event_id_2 = create_event(
        &app,
        &account_2,
        &NewEventForm {
            name: EVENT_NAME_2,
            kind: EventType::Competition,
            start_at: parse_datetime("2021-01-01 00:00:00"),
            end_at: parse_datetime("2021-01-02 00:00:00"),
            venue_id: 2,
            description: "Test Description 2",
            tickets: None,
            registeration_deadline: None,
        },
    )
    .await;

    let req = test::TestRequest::get().uri("/api/event").to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let events: ListEventsResponse = test::read_body_json(resp).await;
    assert_eq!(events.events.len(), 2);
    assert_eq!(events.page.total_item, 2);

    let event = events
        .events
        .iter()
        .find(|e| e.inner.id == event_id_1)
        .unwrap();
    assert_eq!(event.inner.name, EVENT_NAME_1);
    let event = events
        .events
        .iter()
        .find(|e| e.inner.id == event_id_2)
        .unwrap();
    assert_eq!(event.inner.name, EVENT_NAME_2);
}

#[actix_web::test]
async fn test_event_delete() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let event_id = create_default_event(&app, &account).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/event/{}", event_id))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/api/event/{}", event_id))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_event_participate() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let event_id = create_default_event(&app, &account).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/event/{}", event_id))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/api/event/{}", event_id))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
