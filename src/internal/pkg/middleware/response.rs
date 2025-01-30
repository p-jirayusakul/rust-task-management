use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ApiResponseErr {
    pub status: String,
    pub message: String,
}


pub fn response_success<T>(message: &str, data: T) -> ApiResponse<T> {
    ApiResponse {
        status: "success".to_string(),
        message: message.to_string(),
        data,
    }
}

pub fn response_error(message: &str) -> ApiResponseErr {
    ApiResponseErr {
        status: "error".to_string(),
        message: message.to_string(),
    }
}
