use actix_web::{
    body::BoxBody,
    error::ResponseError,
    http::{header::ContentType, header::ToStrError, StatusCode},
    HttpResponse,
};
use argon2::password_hash::errors::Error as PasswordHashError;
use awc::error::{PayloadError, SendRequestError};
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::deadpool::PoolError;
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

use crate::orm::CONSTRAINTS_MAP;

#[derive(Debug, Error, Serialize)]
#[serde(untagged)]
pub enum Error {
    #[error("invalid argument: {0}")]
    InvalidArgument(
        #[from]
        #[serde(skip)]
        ValidationErrors,
    ),

    #[error("Not acceptable: {0}")]
    NotAcceptable(#[serde(skip)] String),

    #[error("Record not found")]
    RecordNotFound { key: Option<String> },

    #[error("Record already exists")]
    RecordAlreadyExists { key: Option<String> },

    #[error("Bad request: {0}")]
    BadRequest(#[serde(skip)] String),

    #[error("Unauthorized: {0}")]
    Unauthorized(#[serde(skip)] String),

    #[error("Internal server error")]
    Internal(#[serde(skip)] InternalError),
}

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("Deadpool: {0}")]
    Deadpool(#[from] PoolError),

    #[error("Diesel: {0}")]
    Diesel(DieselError),

    #[error("PasswordHash: {0}")]
    PasswordHash(#[from] PasswordHashError),

    #[error("awc: SendRequest: {0}")]
    AwcSendRequest(String),

    #[error("awc: Payload: {0}")]
    AwcPayload(#[from] PayloadError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn kind(&self) -> &'static str {
        match self {
            Error::InvalidArgument(_) => "invalid_argument",
            Error::NotAcceptable(_) => "not_acceptable",
            Error::RecordNotFound { .. } => "record_not_found",
            Error::RecordAlreadyExists { .. } => "record_already_exists",
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
    #[serde(flatten)]
    inner: &'a Error,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let message = self.to_string();
        let body = ErrorResponseBody {
            kind: self.kind(),
            message: &message,
            inner: self,
        };

        if let Error::Internal(e) = self {
            error!("Internal server error: {e}");
        }

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(&body)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Error::NotAcceptable(_) => StatusCode::NOT_ACCEPTABLE,
            Error::RecordNotFound { .. } => StatusCode::NOT_FOUND,
            Error::RecordAlreadyExists { .. } => StatusCode::NOT_ACCEPTABLE,
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

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        use diesel::result::{DatabaseErrorKind::*, Error::*};
        trace!("DieselError: {:?}", error);

        fn get_key(constraint_name: Option<&str>) -> Option<String> {
            constraint_name
                .and_then(|s| CONSTRAINTS_MAP.get(s))
                .map(ToString::to_string)
        }

        match error {
            NotFound => Error::RecordNotFound { key: None },
            DatabaseError(UniqueViolation, info) => Error::RecordAlreadyExists {
                key: get_key(info.constraint_name()),
            },
            DatabaseError(ForeignKeyViolation, info) => Error::RecordNotFound {
                key: get_key(info.constraint_name()),
            },
            _ => Error::Internal(InternalError::Diesel(error)),
        }
    }
}

impl From<SendRequestError> for InternalError {
    fn from(error: SendRequestError) -> Self {
        InternalError::AwcSendRequest(error.to_string())
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
