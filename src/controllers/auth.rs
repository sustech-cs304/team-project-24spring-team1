use actix_web::dev::Payload;
use actix_web::http::{header, StatusCode};
use actix_web::web::Buf;
use actix_web::{get, post, web, FromRequest, HttpRequest, HttpResponse, Responder};
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
use uuid::Uuid;
use validator::Validate;

use super::AppState;
use crate::error::{Error, Result};
use crate::orm::account::{Account, AccountCredential, NewAccount, Role};
use crate::orm::utils::RunQueryDsl;
use crate::utils::auth::AuthResult;

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

#[derive(Debug, Serialize)]
struct IdentifierResponse {
    pub identifier: Uuid,
}

#[derive(Debug, Deserialize)]
struct IdentifierQuery {
    pub identifier: Uuid,
}

#[derive(Debug, Deserialize)]
struct CallbackQuery {
    pub identifier: Uuid,
    pub ticket: String,
}

#[get("/identifier")]
async fn get_identifier(state: web::Data<AppState>) -> Result<impl Responder> {
    let identifier = state.auth_provider.lock().unwrap().alloc_identifier();
    log::trace!("New identifier generated: {identifier}");
    Ok(web::Json(IdentifierResponse { identifier }))
}

#[get("/poll")]
async fn poll(
    state: web::Data<AppState>,
    query: web::Query<IdentifierQuery>,
) -> Result<impl Responder> {
    let identifier = query.into_inner().identifier;
    let (tx, rx) = tokio::sync::oneshot::channel();
    state
        .auth_provider
        .lock()
        .unwrap()
        .subscribe(identifier, tx)?;

    log::trace!("Polling for identifier: {identifier}");
    let Ok(result) = rx.await else {
        return Err(Error::NotAcceptable(
            "Current identifier has been unsubscribed".to_owned(),
        ));
    };
    let AccountCredential { id, role, .. } = get_account_or_create(&state, &result).await?;

    debug!("Account logged in: sid={}, id={id}", result.sustech_id);
    let resp = AuthResponse {
        account_id: id,
        token: generate_token(id, role),
    };
    Ok(web::Json(resp))
}

#[get("/callback")]
async fn callback(
    state: web::Data<AppState>,
    query: web::Query<CallbackQuery>,
) -> Result<impl Responder> {
    let CallbackQuery { identifier, ticket } = query.into_inner();
    let result = validate_ticket(&ticket).await?;
    state
        .auth_provider
        .lock()
        .unwrap()
        .callback(identifier, result)?;

    // Return a response that will close the window
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body("<script>window.close();</script>"))
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
        password: Some(&password_hash),
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
    cfg.service(get_identifier)
        .service(poll)
        .service(callback)
        .service(register)
        .service(login)
        .service(check);
}

// ===== Other Functions =====

async fn get_account_or_create(
    state: &web::Data<AppState>,
    auth_result: &AuthResult,
) -> Result<AccountCredential> {
    match Account::by_sustech_id(auth_result.sustech_id)
        .select(AccountCredential::as_select())
        .first(&mut state.pool.get().await?)
        .await
    {
        Ok(credential) => Ok(credential),
        Err(diesel::result::Error::NotFound) => {
            log::debug!(
                "Account not found, creating new account, sid={}, name={}",
                auth_result.sustech_id,
                auth_result.name
            );

            let new_account = NewAccount {
                sustech_id: auth_result.sustech_id,
                name: &auth_result.name,
                email: &auth_result.email,
                password: None,
            };
            Ok(new_account
                .as_insert()
                .returning(AccountCredential::as_select())
                .get_result(&mut state.pool.get().await?)
                .await?)
        }
        Err(err) => Err(err.into()),
    }
}

#[derive(Debug, Serialize)]
struct ValidateQuery<'a> {
    service: &'a str,
    ticket: &'a str,
}

#[derive(Debug, Deserialize)]
struct Value(#[serde(rename = "$text")] String);

#[derive(Debug, Deserialize)]
#[serde(rename = "serviceResponse")]
struct CASResponse {
    #[serde(rename = "authenticationSuccess")]
    authentication: CASAuthentication,
}

#[derive(Debug, Deserialize)]
struct CASAuthentication {
    #[serde(rename = "attributes")]
    attributes: CASAttributes,
    #[serde(rename = "user")]
    user: Value,
}

#[derive(Debug, Deserialize)]
struct CASAttributes {
    #[serde(rename = "mail")]
    email: Value,
    #[serde(rename = "cn")]
    name: Value,
}

async fn validate_ticket(ticket: &str) -> Result<AuthResult> {
    log::trace!("Validating ticket: {ticket}");

    let client = awc::Client::default();
    let mut response = client
        .get("https://sso.cra.ac.cn/realms/cra-service-realm/protocol/cas/serviceValidate")
        .query(&ValidateQuery {
            service: "https://backend.sustech.me",
            ticket,
        })
        .unwrap()
        .send()
        .await?;

    let body = response.body().await?;
    let cas = match quick_xml::de::from_reader::<_, CASResponse>(body.reader()) {
        Ok(cas) => cas.authentication,
        Err(err) => {
            error!("CAS response parse error: {err:?}");
            return Err(Error::BadRequest("CAS response parse error".to_owned()));
        }
    };

    Ok(AuthResult {
        sustech_id: cas.user.0.parse().unwrap(),
        email: cas.attributes.email.0,
        name: cas.attributes.name.0,
    })
}
