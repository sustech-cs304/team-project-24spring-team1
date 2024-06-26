use actix_web::{delete, get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use diesel::prelude::*;
use diesel_async::AsyncConnection;
use scoped_futures::ScopedFutureExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::auth::JwtAuth;
use super::AppState;
use crate::error::{Error, Result};
use crate::orm::event::{
    Event, EventChangeset, EventDisplay, EventSummary, EventType, EventWithParticipation, NewEvent,
};
use crate::orm::misc::Participation;
use crate::orm::schema::{accounts, events, participation};
use crate::orm::utils::types::Point;
use crate::orm::utils::{coalesce, RunQueryDsl};
use crate::orm::utils::{BracketDsl, CountReferencesDsl};
use crate::utils::page::{Page, PaginateQuery};

// ===== Handlers =====

#[derive(Debug, Deserialize, Validate)]
struct NewEventForm {
    pub name: String,
    pub kind: EventType,
    pub description: String,
    pub cover: Option<Uuid>,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: i32,
    pub location: Point,
    pub tickets: Option<i32>,
    pub registration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
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
    pub event_by_id: Option<EventWithParticipation<EventSummary>>,
    pub events: Vec<EventWithParticipation<EventSummary>>,
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
        description: &form.description,
        cover: form.cover,
        start_at: form.start_at,
        end_at: form.end_at,
        venue_id: form.venue_id,
        location: form.location,
        organizer_id: auth.account_id,
        tickets: form.tickets,
        registration_deadline: form.registration_deadline,
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
        let mut sql = Event::all_summary_with_participation().into_boxed();
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
            Some(EventStatus::Applicable) => {
                sql.filter(coalesce(events::registration_deadline, events::end_at).gt(now))
            }
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
        match Event::find_as_summary_with_participation(id)
            .get_result(&mut state.pool.get().await?)
            .await
        {
            Ok(event) => Some(event),
            Err(diesel::result::Error::NotFound) => None,
            Err(e) => return Err(e.into()),
        }
    } else {
        None
    };

    let total_item = as_query()
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;
    let page = Page::builder(total_item, query.page.unwrap_or(1)).build();

    let events = as_query()
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
    let event: EventWithParticipation<EventDisplay> = Event::find_as_display_with_participation(id)
        .get_result(&mut state.pool.get().await?)
        .await?;
    Ok(web::Json(event))
}

#[delete("/{id}")]
async fn delete_event(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let id = path.into_inner();

    let organizer_id = Event::find(id)
        .select(events::organizer_id)
        .get_result::<i32>(&mut state.pool.get().await?)
        .await?;
    if organizer_id != auth.account_id {
        return Err(Error::Unauthorized(
            "You can only delete events you created".into(),
        ));
    }

    let affected = Event::update(id)
        .set(EventChangeset {
            is_deleted: Some(true),
            ..Default::default()
        })
        .execute(&mut state.pool.get().await?)
        .await?;
    // The event is guaranteed to exist
    assert_eq!(affected, 1);

    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

#[post("/{id}/register")]
async fn register_event(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let (deadline, tickets): (NaiveDateTime, Option<i32>) = Event::find(id)
        .select((
            coalesce(events::registration_deadline, events::end_at),
            events::tickets,
        ))
        .get_result(&mut state.pool.get().await?)
        .await?;

    if deadline < Utc::now().naive_utc() {
        return Err(Error::NotAcceptable(
            "Registration deadline has passed".into(),
        ));
    }

    let mut conn = state.pool.get().await?;
    conn.transaction::<_, Error, _>(|conn| {
        async move {
            if let Some(tickets) = tickets {
                let count: i64 = Participation::count_event_participation(id)
                    .get_result(conn)
                    .await?;
                if count >= tickets as i64 {
                    return Err(Error::NotAcceptable("No tickets left".into()));
                }
            }

            Participation::new(auth.account_id, id)
                .as_insert()
                .execute(conn)
                .await?;

            Ok(())
        }
        .scope_boxed()
    })
    .await?;

    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

#[delete("/{id}/register")]
async fn unregister_event(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let id = path.into_inner();
    Participation::new(auth.account_id, id)
        .as_delete()
        .get_result::<Participation>(&mut state.pool.get().await?)
        .await?;
    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

#[get("/participated")]
async fn get_participated(
    state: web::Data<AppState>,
    query: web::Query<PaginateQuery>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let total_item = Participation::by_account_id(auth.account_id)
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;
    let page = Page::builder(total_item, query.page.unwrap_or(1)).build();

    let events = Participation::by_account_id(auth.account_id)
        .order(events::start_at.asc())
        .limit(page.page_size)
        .offset(page.offset)
        .select((
            accounts::id
                .count_references_in(participation::account_id)
                .bracket(),
            EventSummary::as_select(),
        ))
        .get_results(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(ListEventsResponse {
        page,
        event_by_id: None,
        events,
    }))
}

#[get("/{id}/participated")]
async fn get_is_participated(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let count: i64 = participation::table
        .filter(participation::account_id.eq(auth.account_id))
        .filter(participation::event_id.eq(path.into_inner()))
        .count()
        .get_result(&mut state.pool.get().await?)
        .await?;

    match count {
        0 => Ok(HttpResponse::NotFound()),
        1 => Ok(HttpResponse::NoContent()),
        _ => unreachable!("count by primary key should not have multiple results"),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(new_event)
        .service(list_events)
        .service(get_participated)
        .service(get_event)
        .service(get_is_participated)
        .service(delete_event)
        .service(register_event)
        .service(unregister_event);
}
