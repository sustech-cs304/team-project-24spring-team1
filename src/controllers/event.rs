use actix_web::{get, post, web, Responder};
use chrono::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::account::JwtAuth;
use super::AppState;
use crate::error::{Error, Result};
use crate::orm::event::{EventType, NewEvent};
use crate::orm::schema::events;

// ===== Handlers =====

#[derive(Debug, Deserialize, Validate)]
pub struct NewEventForm {
    pub name: String,
    pub kind: EventType,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: i32,
    pub description: String,
    pub tickets: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct EventIdResponse {
    pub id: i32,
}

#[post("/")]
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
    };

    let id = new_event
        .as_insert()
        .returning(events::id)
        .get_result(&mut state.pool.get().await?)
        .await?;

    debug!("New event created: {:?}, id={id}", form.name);
    Ok(web::Json(EventIdResponse { id }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(new_event);
}
