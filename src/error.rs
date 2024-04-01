use actix_web::{
    body::BoxBody,
    error::ResponseError,
    http::{header::ContentType, header::ToStrError, StatusCode},
    HttpResponse,
};
use argon2::password_hash::errors::Error as PasswordHashError;
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::deadpool::PoolError;
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid argument: {0}")]
    InvalidArgument(#[from] ValidationErrors),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error")]
    Internal(InternalError),
}

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("Deadpool: {0}")]
    Deadpool(#[from] PoolError),

    #[error("Diesel: {0}")]
    Diesel(#[from] DieselError),

    #[error("PasswordHash: {0}")]
    PasswordHash(#[from] PasswordHashError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn kind(&self) -> &'static str {
        match self {
            Error::InvalidArgument(_) => "invalid_argument",
            Error::BadRequest(_) => "bad_request",
            Error::Unauthorized(_) => "unauthorized",
            Error::Internal(_) => "internal_server_error",
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponseBody<'a> {
    kind: &'a str,
    message: &'a str,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let message = self.to_string();
        let body = ErrorResponseBody {
            kind: self.kind(),
            message: &message,
        };

        match self {
            Error::Internal(e) => error!("Internal server error: {e}"),
            _ => warn!("API error: {message}"),
        }

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(&body)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<ToStrError> for Error {
    fn from(error: ToStrError) -> Self {
        Error::BadRequest(error.to_string())
    }
}

impl<T> From<T> for Error
where
    T: Into<InternalError>,
{
    fn from(error: T) -> Self {
        Error::Internal(error.into())
    }
}