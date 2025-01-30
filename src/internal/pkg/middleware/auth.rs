use std::future::{ready, Ready};

use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::pkg::middleware::jwt::validate_token;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::LocalBoxFuture;

// Middleware structure
pub struct JwtMiddleware {
    secret: String,
}

impl JwtMiddleware {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

// Middleware factory implementation (Transform)
impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService {
            service,
            secret: self.secret.clone(),
        }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
    secret: String,
}

// Middleware service implementation
impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Example: Extract header `Authorization`
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok());

        // Validate JWT
        if let Some(auth_header) = auth_header {
            if !auth_header.starts_with("Bearer ") {
                return Box::pin(async { return Err(Error::from(CustomError::Unauthorized("Invalid Authorization header".to_string()))); });
            }

            let token = &auth_header[7..];

            match validate_token(token, self.secret.as_str()) {
                Ok(token_data) => {
                    req.extensions_mut().insert(token_data.claims.sub);
                }
                Err(_) => {
                    return Box::pin(async { return Err(Error::from(CustomError::Unauthorized("Invalid JWT Token".to_string()))); });
                }
            }
        } else {
            return Box::pin(async { Err(Error::from(CustomError::Unauthorized("No Authorization header found".to_string()))) });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}