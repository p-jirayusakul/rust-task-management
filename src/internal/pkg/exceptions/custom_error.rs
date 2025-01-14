use crate::internal::pkg::middleware::response::response_error;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use std::fmt::{Display, Formatter};
#[allow(dead_code)]
#[derive(Debug)]
pub enum MyError {
    ValidationError(String),
    InternalError(String),
    DomainError(String),
    BusinessError(String),
    SystemError(String),
    NotFound(String),
    RepositoryError(String),
    UnknownError(String),
    Unauthorized(String),
    Forbidden(String),
    DataConflict(String),
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::ValidationError(message) => {
                write!(f, "{}", message)
            }
            MyError::InternalError(message) => {
                write!(f, "{}", message)
            }
            MyError::DomainError(message) => {
                write!(f, "{}", message)
            }
            MyError::BusinessError(message) => {
                write!(f, "{}", message)
            }
            MyError::SystemError(message) => {
                write!(f, "{}", message)
            }
            MyError::NotFound(message) => {
                write!(f, "{}", message)
            }
            MyError::RepositoryError(message) => {
                write!(f, "{}", message)
            }
            MyError::UnknownError(message) => {
                write!(f, "{}", message)
            }
            MyError::Unauthorized(message) => {
                write!(f, "{}", message)
            }
            MyError::Forbidden(message) => {
                write!(f, "{}", message)
            }
            MyError::DataConflict(message) => {
                write!(f, "{}", message)
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            MyError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::DomainError { .. } => StatusCode::BAD_REQUEST,
            MyError::BusinessError { .. } => StatusCode::BAD_REQUEST,
            MyError::SystemError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound { .. } => StatusCode::NOT_FOUND,
            MyError::RepositoryError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::UnknownError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            MyError::Forbidden { .. } => StatusCode::FORBIDDEN,
            MyError::DataConflict { .. } => StatusCode::CONFLICT,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(response_error(&self.to_string()))
    }
}