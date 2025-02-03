use actix_web::web;
use crate::application::interfaces::auth::AuthUseCase;
use crate::infrastructure::api::handlers::auth::AuthHandler;

pub fn configure_user_routes<T: AuthUseCase + Send + Sync + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/users")
            .route("/login", web::post().to(AuthHandler::<T>::login))
    );
}