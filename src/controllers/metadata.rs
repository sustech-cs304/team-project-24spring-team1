use actix_web::{get, web, Responder};
use diesel_async::RunQueryDsl;

use super::AppState;
use crate::error::Result;
use crate::orm::misc::Place;

// ===== Handlers =====

#[get("/places")]
async fn places(state: web::Data<AppState>) -> Result<impl Responder> {
    let places = Place::all()
        .get_results(&mut state.pool.get().await?)
        .await?;
    Ok(web::Json(places))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(places);
}
