use actix_web::{web, HttpResponse, Responder};
use crate::application::interfaces::health_check::HealthCheckUseCase;
use crate::shared::exceptions::custom_error::CustomError;
use crate::shared::middleware::response::response_success;

pub struct HealthCheckHandler<T: HealthCheckUseCase + Send + Sync> {
    use_case: T,
}

impl<T: HealthCheckUseCase + Send + Sync> HealthCheckHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    pub(crate) async fn live() -> impl Responder {
        HttpResponse::Ok().json(response_success("OK", ()))
    }

    pub(crate) async fn ready(
        handler: web::Data<HealthCheckHandler<T>>,
    ) -> Result<impl Responder, CustomError> {
        match handler.use_case.readiness().await {
            Ok(..) => Ok(HttpResponse::Ok().json(response_success("OK", ()))),
            Err(e) => Err(e)
        }
    }
}