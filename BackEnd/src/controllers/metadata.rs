use actix_web::{get, web, Responder};

use super::AppState;
use crate::error::Result;
use crate::orm::misc::Place;
use crate::orm::utils::RunQueryDsl;

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
