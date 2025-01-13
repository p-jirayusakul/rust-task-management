use actix_web::http::StatusCode;
use std::fmt;
use std::error::Error;

/// Enum representing various error statuses.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    DomainError,
    BusinessError,
    SystemError,
    NotFound,
    RepositoryError,
    UnknownError,
    Unauthorized,
    Forbidden,
    DataConflict,
}

/// Struct for custom errors with a status and a detailed message.
#[derive(Debug)]
pub struct MyError {
    pub status: Status,
    pub message: String,
}

impl MyError {
    pub fn new(status: Status, message: impl Into<String>) -> Self {
        MyError {
            status,
            message: message.into(),
        }
    }

    pub fn http_status_code(&self) -> StatusCode {
        match self.status {
            Status::DomainError | Status::BusinessError => StatusCode::BAD_REQUEST, // 400
            Status::SystemError | Status::UnknownError | Status::RepositoryError => StatusCode::INTERNAL_SERVER_ERROR, // 500
            Status::NotFound => StatusCode::NOT_FOUND, // 404
            Status::Unauthorized => StatusCode::UNAUTHORIZED, // 401
            Status::Forbidden => StatusCode::FORBIDDEN, // 403
            Status::DataConflict => StatusCode::CONFLICT, // 409
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

impl Error for MyError {}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Status::DomainError => "Domain Error",
            Status::BusinessError => "Business Error",
            Status::SystemError => "System Error",
            Status::NotFound => "Not Found",
            Status::RepositoryError => "Repository Error",
            Status::UnknownError => "Unknown Error",
            Status::Unauthorized => "Unauthorized",
            Status::Forbidden => "Forbidden",
            Status::DataConflict => "Data Conflict",
        };
        write!(f, "{}", status_str)
    }
}
