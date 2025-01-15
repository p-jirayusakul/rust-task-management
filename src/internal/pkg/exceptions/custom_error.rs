use crate::internal::pkg::middleware::response::response_error;
use actix_web::{error, http::header::ContentType, http::StatusCode, HttpResponse};
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
pub enum CustomError {
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

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::ValidationError(message) => {
                write!(f, "{}", message)
            }
            CustomError::InternalError(message) => {
                write!(f, "{}", message)
            }
            CustomError::DomainError(message) => {
                write!(f, "{}", message)
            }
            CustomError::BusinessError(message) => {
                write!(f, "{}", message)
            }
            CustomError::SystemError(message) => {
                write!(f, "{}", message)
            }
            CustomError::NotFound(message) => {
                write!(f, "{}", message)
            }
            CustomError::RepositoryError(message) => {
                write!(f, "{}", message)
            }
            CustomError::UnknownError(message) => {
                write!(f, "{}", message)
            }
            CustomError::Unauthorized(message) => {
                write!(f, "{}", message)
            }
            CustomError::Forbidden(message) => {
                write!(f, "{}", message)
            }
            CustomError::DataConflict(message) => {
                write!(f, "{}", message)
            }
        }
    }
}

impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::DomainError { .. } => StatusCode::BAD_REQUEST,
            CustomError::BusinessError { .. } => StatusCode::BAD_REQUEST,
            CustomError::SystemError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::NotFound { .. } => StatusCode::NOT_FOUND,
            CustomError::RepositoryError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::UnknownError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            CustomError::Forbidden { .. } => StatusCode::FORBIDDEN,
            CustomError::DataConflict { .. } => StatusCode::CONFLICT,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(response_error(&self.to_string()))
    }
}
