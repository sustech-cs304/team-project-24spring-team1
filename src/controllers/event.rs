use actix_web::{get, post, web, Responder};
use chrono::prelude::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::account::JwtAuth;
use super::AppState;
use crate::error::Result;
use crate::orm::event::{Event, EventDisplay, EventSummary, EventType, NewEvent};
use crate::orm::schema::{accounts, events, places};
use crate::utils::page::Page;

// ===== Handlers =====

#[derive(Debug, Deserialize, Validate)]
struct NewEventForm {
    pub name: String,
    pub kind: EventType,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: i32,
    pub description: String,
    pub tickets: Option<i32>,
    pub registeration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EventStatus {
    Applicable,
    NotStarted,
    Ongoing,
    Ended,
}

#[derive(Debug, Deserialize, Validate)]
struct EventFilterQuery {
    pub name: Option<String>,
    pub kind: Option<EventType>,
    pub venue_id: Option<i32>,
    pub organizer_id: Option<i32>,
    pub status: Option<EventStatus>,
    pub page: Option<i64>,
}

#[derive(Debug, Serialize)]
struct EventIdResponse {
    pub id: i32,
}

#[derive(Debug, Serialize)]
struct ListEventsResponse {
    pub page: Page,
    pub event_by_id: Option<EventSummary>,
    pub events: Vec<EventSummary>,
}

#[post("")]
async fn new_event(
    state: web::Data<AppState>,
    form: web::Json<NewEventForm>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let form = form.into_inner();
    form.validate()?;

    let new_event = NewEvent {
        name: &form.name,
        kind: form.kind,
        start_at: form.start_at,
        end_at: form.end_at,
        venue_id: form.venue_id,
        description: &form.description,
        organizer_id: auth.user_id,
        tickets: form.tickets,
        registeration_deadline: form.registeration_deadline,
    };

    let id = new_event
        .as_insert()
        .returning(events::id)
        .get_result(&mut state.pool.get().await?)
        .await?;

    debug!("New event created: {:?}, id={id}", form.name);
    Ok(web::Json(EventIdResponse { id }))
}

#[get("")]
async fn list_events(
    state: web::Data<AppState>,
    query: web::Query<EventFilterQuery>,
) -> Result<impl Responder> {
    let query = query.into_inner();
    query.validate()?;

    let now = Utc::now().naive_utc();

    let as_query = || {
        let mut sql = Event::all().into_boxed();
        if let Some(name) = &query.name {
            sql = sql.filter(events::name.like(format!("%{name}%")));
        }
        if let Some(kind) = &query.kind {
            sql = sql.filter(events::kind.eq(kind));
        }
        if let Some(venue_id) = &query.venue_id {
            sql = sql.filter(events::venue_id.eq(venue_id));
        }
        if let Some(organizer_id) = &query.organizer_id {
            sql = sql.filter(events::organizer_id.eq(organizer_id));
        }

        sql = match query.status {
            Some(EventStatus::Applicable) => sql.filter(events::registeration_deadline.gt(now)),
            Some(EventStatus::NotStarted) => sql.filter(events::start_at.gt(now)),
            Some(EventStatus::Ongoing) => sql
                .filter(events::start_at.le(now))
                .filter(events::end_at.gt(now)),
            Some(EventStatus::Ended) => sql.filter(events::end_at.le(now)),
            None => sql,
        };

        sql
    };

    let event_by_id = if let Some(id) = query.name.as_ref().and_then(|name| name.parse().ok()) {
        let event = Event::find_as_summary(id)
            .get_result(&mut state.pool.get().await?)
            .await?;
        Some(event)
    } else {
        None
    };

    let total_item = as_query()
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;
    let page = Page::builder(total_item, query.page.unwrap_or(1)).build();

    let events = as_query()
        .inner_join(accounts::table)
        .inner_join(places::table)
        .select(EventSummary::as_select())
        .order(events::start_at.asc())
        .limit(page.page_size)
        .offset(page.offset)
        .get_results(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(ListEventsResponse {
        page,
        event_by_id,
        events,
    }))
}

#[get("/{id}")]
async fn get_event(state: web::Data<AppState>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let event: EventDisplay = Event::find_as_display(id)
        .get_result(&mut state.pool.get().await?)
        .await?;
    Ok(web::Json(event))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(new_event)
        .service(list_events)
        .service(get_event);
}