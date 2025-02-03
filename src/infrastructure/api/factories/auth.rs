use std::sync::Arc;
use actix_web::web;
use deadpool_postgres::Pool;
use crate::application::use_cases::auth::AuthUseCaseImpl;
use crate::infrastructure::api::handlers::auth::AuthHandler;
use crate::infrastructure::config::ServerConfig;
use crate::infrastructure::database::auth::AuthRepositoriesImpl;

// ฟังก์ชันสำหรับสร้าง Auth Handler
pub fn create_user_handler_data(
    pool: Arc<Pool>,
    config: &ServerConfig,
) -> web::Data<AuthHandler<AuthUseCaseImpl<AuthRepositoriesImpl>>> {
    let user_repository = AuthRepositoriesImpl::new(pool);
    let user_use_case = AuthUseCaseImpl::new(user_repository, config.jwt_secret.clone()); // UseCase logic
    let user_handler = AuthHandler::new(user_use_case);
    web::Data::new(user_handler)
}
