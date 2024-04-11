use actix_web::{get, put, web, Responder};
use diesel::prelude::*;
use serde::Deserialize;
use validator::Validate;

use super::auth::JwtAuth;
use super::AppState;
use crate::error::Result;
use crate::orm::account::{Account, AccountProfile};
use crate::orm::schema::accounts;
use crate::orm::utils::RunQueryDsl;

// ===== Handlers =====

#[derive(Debug, Deserialize, Validate)]
struct UpdateProfileForm {
    pub bio: String,
}

#[get("/{id}/profile")]
async fn get_profile(state: web::Data<AppState>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let account: AccountProfile = Account::find(id)
        .select(AccountProfile::as_select())
        .first(&mut state.pool.get().await?)
        .await?;

    Ok(web::Json(account))
}

#[put("/self/profile")]
async fn update_profile(
    state: web::Data<AppState>,
    form: web::Json<UpdateProfileForm>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    let form = form.into_inner();
    form.validate()?;

    let affected = Account::update(auth.account_id)
        .set(accounts::bio.eq(&form.bio))
        .execute(&mut state.pool.get().await?)
        .await?;
    assert_eq!(affected, 1);

    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(update_profile).service(get_profile);
}
