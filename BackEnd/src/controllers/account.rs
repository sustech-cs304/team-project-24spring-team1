use actix_web::{get, put, web, Responder};
use diesel::prelude::*;
use serde::Deserialize;
use std::borrow::Cow;
use std::result::Result as StdResult;
use uuid::Uuid;
use validator::{Validate, ValidateArgs, ValidationError};

use super::auth::JwtAuth;
use super::AppState;
use crate::error::{Error, Result};
use crate::orm::account::{Account, AccountProfile, AccountProfileChangeset, Role};
use crate::orm::utils::RunQueryDsl;

// ===== Handlers =====

#[derive(Debug, Deserialize, Validate)]
#[validate(context = "Role")]
struct UpdateProfileForm {
    #[validate(email)]
    #[validate(custom(function = "validate_optional_field_access", use_context))]
    pub email: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<Uuid>,
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

#[put("/{id}/profile")]
async fn update_profile(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    form: web::Json<UpdateProfileForm>,
    auth: JwtAuth,
) -> Result<impl Responder> {
    if path.into_inner() != auth.account_id && !auth.role.has_access_to(Role::Admin) {
        return Err(Error::Unauthorized(
            "You can only update your own profile".to_owned(),
        ));
    }

    let form = form.into_inner();
    form.validate_with_args(&auth.role)?;

    let affected = Account::update(auth.account_id)
        .set(form.as_changeset())
        .execute(&mut state.pool.get().await?)
        .await?;
    // The account is guaranteed to exist
    assert_eq!(affected, 1);

    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(update_profile).service(get_profile);
}

// ===== Other Functions =====

impl UpdateProfileForm {
    pub fn as_changeset(&self) -> AccountProfileChangeset {
        AccountProfileChangeset {
            email: self.email.as_deref(),
            bio: self.bio.as_deref(),
            avatar: self.avatar,
        }
    }
}

fn validate_optional_field_access<T>(_value: &T, role: &Role) -> StdResult<(), ValidationError> {
    if !role.has_access_to(Role::Staff) {
        return Err(ValidationError::new("unauthorized")
            .with_message(Cow::from("You are not allowed to set this field")));
    }
    Ok(())
}
