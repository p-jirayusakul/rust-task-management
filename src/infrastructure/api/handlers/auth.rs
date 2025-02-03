use actix_web::{web, HttpResponse, Responder};
use validator::Validate;
use crate::application::interfaces::auth::AuthUseCase;
use crate::domain::entities::auth::Login;
use crate::infrastructure::api::requests::auth::LoginRequest;
use crate::shared::exceptions::custom_error::CustomError;
use crate::shared::middleware::response::response_success;

pub struct AuthHandler<T: AuthUseCase + Send + Sync> {
    use_case: T,
}

impl<T: AuthUseCase + Send + Sync> AuthHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    pub(crate) async fn login(
        handler: web::Data<AuthHandler<T>>,
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
