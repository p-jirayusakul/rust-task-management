use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::pkg::middleware::response::response_success;
use crate::internal::server::domain::entities::user::Login;
use crate::internal::server::domain::use_case::user::UserUseCase;
use crate::internal::server::request::user::LoginRequest;
use actix_web::{web, HttpResponse, Responder};
use validator::Validate;

pub struct UserHandler<T: UserUseCase + Send + Sync> {
    use_case: T,
}

impl<T: UserUseCase + Send + Sync> UserHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    async fn login(
        handler: web::Data<UserHandler<T>>,
        body: web::Json<LoginRequest>,
    ) -> Result<impl Responder, CustomError> {
        body.validate().map_err(|e| CustomError::ValidationError(e.to_string()))?;

        let payload = Login {
            username: body.username.clone(),
            password: body.password.clone(),
        };

        match handler.use_case.login(payload).await {
            Ok(token) => Ok(HttpResponse::Ok().json(response_success("login successfully", token))),
            Err(e) => Err(e)
        }
    }
}

pub fn configure_routes<T: UserUseCase + Send + Sync + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/users")
            .route("/login", web::post().to(UserHandler::<T>::login))
    );
}