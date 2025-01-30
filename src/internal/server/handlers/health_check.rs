use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::pkg::middleware::response::response_success;
use crate::internal::server::domain::use_case::health_check::HealthCheckUseCase;
use actix_web::{web, HttpResponse, Responder};

pub struct HealthCheckHandler<T: HealthCheckUseCase + Send + Sync> {
    use_case: T,
}

impl<T: HealthCheckUseCase + Send + Sync> HealthCheckHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    async fn live() -> impl Responder {
        HttpResponse::Ok().json(response_success("OK", ()))
    }

    async fn ready(
        handler: web::Data<HealthCheckHandler<T>>,
    ) -> Result<impl Responder, CustomError> {
        match handler.use_case.readiness().await {
            Ok(..) => Ok(HttpResponse::Ok().json(response_success("OK", ()))),
            Err(e) => Err(e)
        }
    }
}

pub fn configure_routes<T: HealthCheckUseCase + Send + Sync + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/health-check")
            .route("/live", web::get().to(HealthCheckHandler::<T>::live))
            .route("/ready", web::get().to(HealthCheckHandler::<T>::ready))
    );
}