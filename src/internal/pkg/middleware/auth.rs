use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::error;
use crate::internal::pkg::middleware::response::response_error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (e.g., user id)
    exp: usize,  // Expiry timestamp
}

// JWT Authentication Middleware
pub struct JwtMiddleware {
    secret: String,
}

impl JwtMiddleware {
    pub fn new(secret: &str) -> Self {
        JwtMiddleware {
            secret: secret.to_owned(),
        }
    }
}


impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service, secret: self.secret.clone() }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        // Clone variables outside future.
        let secret = self.secret.clone();
        let headers = req.headers().clone();
        let token_opt = headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .map(|header| header.trim_start_matches("Bearer "));

        let validation_result = if let Some(token) = token_opt {
            decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            )
                .map(|decoded| decoded.claims)
                .ok()
        } else {
            None
        };

        // Fail if validation fails.
        if validation_result.is_none() {
            let fut = self.service.call(req);
            return  Box::pin(async move {
                let res = fut.await?;

                let (req, _) = res.into_parts();
                let bb = String::from("Unauthorized");

                let res = res.set_body(bb);
                let res = ServiceResponse::new(req, res);

                Ok(res)
            })
        }

        // Attach valid claims to request extensions.
        let claims = validation_result.unwrap();
        req.extensions_mut().insert(claims);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}