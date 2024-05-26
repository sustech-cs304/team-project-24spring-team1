use actix_web::http::StatusCode;
use actix_web::test;
use chrono::{Days, NaiveDateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point(pub f64, pub f64);

#[derive(Debug, Serialize)]
pub struct NewEventForm<'a> {
    pub name: &'a str,
    pub kind: EventType,
    pub description: &'a str,
    pub cover: Option<Uuid>,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: i32,
    pub location: Point,
    pub tickets: Option<i32>,
    pub registration_deadline: Option<NaiveDateTime>,
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
    pub description: String,
    pub cover: Uuid,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue: Place,
    pub location: Point,
    pub organizer: AccountCard,
    pub tickets: Option<i32>,
    pub registration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventSummaryResponse {
    pub id: i32,
    pub name: String,
    pub kind: EventType,
    pub description: String,
    pub cover: Uuid,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue: Place,
    pub location: Point,
    pub organizer: AccountCard,
    pub tickets: Option<i32>,
    pub registration_deadline: Option<NaiveDateTime>,
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

pub async fn get_event(
    app: impl TestApp,
    event_id: i32,
) -> EventWithParticipation<EventDisplayResponse> {
    let req = test::TestRequest::get()
        .uri(&format!("/api/event/{event_id}"))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    test::read_body_json(resp).await
}

lazy_static! {
    static ref DEFAULT_EVENT_1: NewEventForm<'static> = NewEventForm {
        name: "Test Event 1",
        kind: EventType::Show,
        description: "Test Description",
        cover: None,
        start_at: parse_datetime("2021-01-01 00:00:00"),
        end_at: parse_datetime("2021-01-02 00:00:00"),
        venue_id: 1,
        location: Point(0.0, 0.0),
        tickets: None,
        registration_deadline: None,
    };
    static ref DEFAULT_EVENT_2: NewEventForm<'static> = NewEventForm {
        name: "Test Event 2",
        kind: EventType::Lecture,
        description: "Test Description 2",
        cover: None,
        start_at: parse_datetime("2021-01-01 00:00:00"),
        end_at: parse_datetime("2021-01-02 00:00:00"),
        venue_id: 2,
        location: Point(0.0, 0.0),
        tickets: None,
        registration_deadline: None,
    };
}

pub async fn create_default_event(app: impl TestApp, account: &AccountInfo) -> i32 {
    create_event(app, account, &DEFAULT_EVENT_1).await
}

#[actix_web::test]
async fn test_event_create_and_get() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let event_id = create_default_event(&app, &account).await;

    let event = get_event(&app, event_id).await;
    assert_eq!(event.inner.id, event_id);
    assert_eq!(event.inner.name, DEFAULT_EVENT_1.name);
    assert_eq!(event.inner.kind, DEFAULT_EVENT_1.kind);
    assert_eq!(event.inner.start_at, DEFAULT_EVENT_1.start_at);
    assert_eq!(event.inner.end_at, DEFAULT_EVENT_1.end_at);
    assert_eq!(event.inner.venue.id, DEFAULT_EVENT_1.venue_id);
    assert_eq!(event.inner.description, DEFAULT_EVENT_1.description);
    assert_eq!(event.inner.organizer.id, account.account_id);
}

#[actix_web::test]
async fn test_event_list() {
    let app = create_app().await;

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

    let event_id_1 = create_event(&app, &account_1, &DEFAULT_EVENT_1).await;
    let event_id_2 = create_event(&app, &account_2, &DEFAULT_EVENT_2).await;

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
    assert_eq!(event.inner.name, DEFAULT_EVENT_1.name);
    let event = events
        .events
        .iter()
        .find(|e| e.inner.id == event_id_2)
        .unwrap();
    assert_eq!(event.inner.name, DEFAULT_EVENT_2.name);

    async fn test_filter(app: impl TestApp, filter: &str, expected: i32) {
        let req = test::TestRequest::get()
            .uri(&format!("/api/event?{filter}"))
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let events: ListEventsResponse = test::read_body_json(resp).await;
        assert_eq!(events.events.len(), 1);
        assert_eq!(events.events[0].inner.id, expected);
    }

    test_filter(&app, "name=+2", event_id_2).await;
    test_filter(&app, "kind=lecture", event_id_2).await;

    test_filter(
        &app,
        &format!("venue_id={}", DEFAULT_EVENT_1.venue_id),
        event_id_1,
    )
    .await;

    test_filter(
        &app,
        &format!("organizer_id={}", account_1.account_id),
        event_id_1,
    )
    .await;
}

#[actix_web::test]
async fn test_event_filter_status() {
    let app = create_app().await;
    let account = create_default_account(&app).await;

    let now = Utc::now().naive_utc();
    let before_time = now.checked_sub_days(Days::new(1)).unwrap();
    let after_time = now.checked_add_days(Days::new(1)).unwrap();

    let event_id_1 = create_event(
        &app,
        &account,
        &NewEventForm {
            registration_deadline: Some(before_time),
            start_at: before_time,
            end_at: before_time,
            ..*DEFAULT_EVENT_1
        },
    )
    .await;
    let event_id_2 = create_event(
        &app,
        &account,
        &NewEventForm {
            registration_deadline: Some(before_time),
            start_at: before_time,
            end_at: after_time,
            ..*DEFAULT_EVENT_1
        },
    )
    .await;
    let event_id_3 = create_event(
        &app,
        &account,
        &NewEventForm {
            registration_deadline: Some(before_time),
            start_at: after_time,
            end_at: after_time,
            ..*DEFAULT_EVENT_1
        },
    )
    .await;
    let event_id_4 = create_event(
        &app,
        &account,
        &NewEventForm {
            registration_deadline: Some(after_time),
            start_at: after_time,
            end_at: after_time,
            ..*DEFAULT_EVENT_1
        },
    )
    .await;
    let event_id_5 = create_event(
        &app,
        &account,
        &NewEventForm {
            registration_deadline: None,
            start_at: before_time,
            end_at: after_time,
            ..*DEFAULT_EVENT_1
        },
    )
    .await;

    async fn test_filter(app: impl TestApp, status: &str, expected: &[i32]) {
        let expected = expected.iter().cloned().collect::<HashSet<_>>();

        let req = test::TestRequest::get()
            .uri(&format!("/api/event?status={status}"))
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let events: ListEventsResponse = test::read_body_json(resp).await;
        let events_id = events
            .events
            .iter()
            .map(|e| e.inner.id)
            .collect::<HashSet<_>>();
        assert_eq!(events_id, expected);
    }

    test_filter(&app, "applicable", &[event_id_4, event_id_5]).await;
    test_filter(&app, "not_started", &[event_id_3, event_id_4]).await;
    test_filter(&app, "ongoing", &[event_id_2, event_id_5]).await;
    test_filter(&app, "ended", &[event_id_1]).await;
}

#[actix_web::test]
async fn test_event_find_by_id() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let event_id = create_default_event(&app, &account).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/event?name={event_id}"))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let events: ListEventsResponse = test::read_body_json(resp).await;
    assert!(events.event_by_id.is_some());
    assert_eq!(events.event_by_id.unwrap().inner.name, DEFAULT_EVENT_1.name);

    // unexist id should not fail the whole request
    let req = test::TestRequest::get()
        .uri(&format!("/api/event?name={}", event_id + 1))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_event_delete() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let event_id = create_default_event(&app, &account).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/event/{event_id}"))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/api/event/{event_id}"))
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_event_participate() {
    let app = create_app().await;
    let account = create_default_account(&app).await;

    let now = Utc::now().naive_utc();
    let event_id = create_event(
        &app,
        &account,
        &NewEventForm {
            registration_deadline: Some(now.checked_add_days(Days::new(1)).unwrap()),
            ..*DEFAULT_EVENT_1
        },
    )
    .await;

    // REGISTER
    let req = test::TestRequest::post()
        .uri(&format!("/api/event/{event_id}/register"))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // already registered
    // let req = test::TestRequest::post()
    //     .uri(&format!("/api/event/{event_id}/register"))
    //     .insert_header(account.to_header_pair())
    //     .to_request();
    // let resp = app.call(req).await.unwrap();
    // assert_eq!(resp.status(), StatusCode::NOT_ACCEPTABLE);

    let event = get_event(&app, event_id).await;
    assert_eq!(event.participation_count, 1);

    let req = test::TestRequest::get()
        .uri("/api/event/participated")
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let events: ListEventsResponse = test::read_body_json(resp).await;
    assert_eq!(events.events[0].inner.id, event_id);

    let req = test::TestRequest::get()
        .uri(&format!("/api/event/{event_id}/participated"))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // UNREGISTER
    let req = test::TestRequest::delete()
        .uri(&format!("/api/event/{event_id}/register"))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let event = get_event(&app, event_id).await;
    assert_eq!(event.participation_count, 0);

    let req = test::TestRequest::get()
        .uri(&format!("/api/event/{event_id}/participated"))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // not registered
    let req = test::TestRequest::delete()
        .uri(&format!("/api/event/{event_id}/register"))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_event_deadline_passed() {
    let app = create_app().await;
    let account = create_default_account(&app).await;
    let event_id = create_default_event(&app, &account).await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/event/{event_id}/register"))
        .insert_header(account.to_header_pair())
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_ACCEPTABLE);
}
