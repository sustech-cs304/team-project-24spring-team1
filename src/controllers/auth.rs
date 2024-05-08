use actix_web::dev::Payload;
use actix_web::http::header;
use actix_web::{get, post, web, FromRequest, HttpRequest, Responder};
use argon2::{
    password_hash::{
        errors::Error as PasswordHashError, rand_core::OsRng, PasswordHash, PasswordHasher,
        PasswordVerifier, SaltString,
    },
    Argon2,
};
use chrono::{prelude::*, Duration};
use diesel::prelude::*;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use validator::Validate;

use super::AppState;
use crate::error::{Error, Result};
use crate::orm::account::{Account, AccountCredential, NewAccount, Role};
use crate::orm::utils::RunQueryDsl;

lazy_static! {
    static ref JWT_SECRET: String = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        warn!("JWT_SECRET is not set, using default value");
        "123456".to_string()
    });
}
const ALGORITHM: Algorithm = Algorithm::HS256;

pub struct JwtAuth {
    pub account_id: i32,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    pub sub: i32,
    pub role: Role,

    pub iat: usize,
    pub exp: usize,
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

fn verify_password(password: &str, password_hash: &str) -> Result<()> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    match result {
        Err(PasswordHashError::Password) => {
            Err(Error::Unauthorized(String::from("password incorrect")))
        }
        result => Ok(result?),
    }
}

fn generate_token(account_id: i32, role: Role) -> String {
    let now = Utc::now();
    let claims: TokenClaims = TokenClaims {
        sub: account_id,
        role,
        exp: (now + Duration::hours(24)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    jsonwebtoken::encode(
        &Header::new(ALGORITHM),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .unwrap()
}

fn request_to_jwt_middleware(req: &HttpRequest) -> Result<JwtAuth> {
    const SCHEMA: &str = "Bearer ";
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .map(|value| value.to_str())
        .transpose()?
        .and_then(|value| {
            value
                .starts_with(SCHEMA)
                .then_some(value.split_at(SCHEMA.len()).1)
        })
        .ok_or_else(|| {
            Error::Unauthorized(String::from(
                "A valid header \"Authorization\" must be provided for authorization",
            ))
        })?;

    let claims = jsonwebtoken::decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::new(ALGORITHM),
    )
    .map_err(|err| {
        trace!("JWT decode error: {:?}", err);
        Error::Unauthorized(String::from("Invalid token"))
    })?;

    Ok(JwtAuth {
        account_id: claims.claims.sub,
        role: claims.claims.role,
    })
}

impl FromRequest for JwtAuth {
    type Error = Error;
    type Future = Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(request_to_jwt_middleware(req))
    }
}

// ===== Handlers =====

#[derive(Debug, Deserialize, Validate)]
struct RegisterForm {
    #[validate(range(min = 10000000, max = 99999999))]
    sustech_id: i32,

    #[validate(length(min = 1, max = 30))]
    name: String,

    #[validate(length(min = 8, max = 64))]
    password: String,
}

#[derive(Debug, Deserialize, Validate)]
struct LoginForm {
    #[validate(range(min = 10000000, max = 99999999))]
    sustech_id: i32,

    #[validate(length(min = 8, max = 64))]
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    pub account_id: i32,
    pub token: String,
}

#[post("/register")]
async fn register(
    state: web::Data<AppState>,
    form: web::Json<RegisterForm>,
) -> Result<impl Responder> {
    let form = form.into_inner();
    form.validate()?;

    let password_hash = hash_password(&form.password)?;
    let default_email = format!("{}@mail.sustech.edu.cn", form.sustech_id);
    let new_account = NewAccount {
        sustech_id: form.sustech_id,
        name: &form.name,
        email: &default_email,
        password: &password_hash,
    };

    let AccountCredential { id, role, .. } = new_account
        .as_insert()
        .returning(AccountCredential::as_select())
        .get_result(&mut state.pool.get().await?)
        .await?;

    debug!(
        "New account created: {:?}, sid={}, id={id}",
        form.name, form.sustech_id
    );
    let resp = AuthResponse {
        account_id: id,
        token: generate_token(id, role),
    };
    Ok(web::Json(resp))
}

#[post("/login")]
async fn login(state: web::Data<AppState>, form: web::Json<LoginForm>) -> Result<impl Responder> {
    let form = form.into_inner();
    form.validate()?;

    let AccountCredential { id, password, role } = Account::by_sustech_id(form.sustech_id)
        .select(AccountCredential::as_select())
        .first(&mut state.pool.get().await?)
        .await?;
    verify_password(&form.password, &password)?;

    debug!("Account logged in: sid={}, id={id}", form.sustech_id);
    let resp = AuthResponse {
        account_id: id,
        token: generate_token(id, role),
    };
    Ok(web::Json(resp))
}

#[get("/check")]
async fn check(_auth: JwtAuth) -> Result<impl Responder> {
    Ok(web::Json(serde_json::Value::Object(Default::default())))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register).service(login).service(check);
}
