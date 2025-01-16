use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, Error};
use std::env;
use actix_web::body::BoxBody;
use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::pkg::middleware::jwt::validate_token;

pub async fn jwt_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {

    // pre-processing
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        if !auth_header.starts_with("Bearer ") {
            return Err(Error::from(CustomError::Unauthorized("Invalid Authorization header".to_string())));

        }

        let token = &auth_header[7..];
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment variables");
        if let Err(_) = validate_token(token, jwt_secret.as_str()) {
            return Err(Error::from(CustomError::Unauthorized("Invalid JWT Token".to_string())));
        }
    } else {
        return Err(Error::from(CustomError::Unauthorized("No Authorization header found".to_string())));
    }

    next.call(req).await
}