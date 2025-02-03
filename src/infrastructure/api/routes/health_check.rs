use actix_web::web;
use crate::application::interfaces::health_check::HealthCheckUseCase;
use crate::infrastructure::api::handlers::health_check::HealthCheckHandler;

pub fn config_health_check_routes<T: HealthCheckUseCase + Send + Sync + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/health-check")
            .route("/live", web::get().to(HealthCheckHandler::<T>::live))
            .route("/ready", web::get().to(HealthCheckHandler::<T>::ready))
    );
}