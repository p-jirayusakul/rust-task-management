use std::sync::Arc;
use actix_web::web;
use deadpool_postgres::Pool;
use crate::application::use_cases::health_check::HealthCheckUseCaseImpl;
use crate::infrastructure::api::handlers::health_check::HealthCheckHandler;
use crate::infrastructure::database::health_check::HealthCheckRepositoriesImpl;

// ฟังก์ชันสำหรับสร้าง Health Check Data Handler
pub fn create_health_check_handler_data(
    pool: Arc<Pool>,
) -> web::Data<HealthCheckHandler<HealthCheckUseCaseImpl<HealthCheckRepositoriesImpl>>> {
    let health_check_repository = HealthCheckRepositoriesImpl::new(pool);
    let health_check_use_case = HealthCheckUseCaseImpl::new(health_check_repository);
    let health_check_handler = HealthCheckHandler::new(health_check_use_case);
    web::Data::new(health_check_handler)
}
