use async_trait::async_trait;
use bcrypt::verify;
use crate::application::interfaces::auth::AuthUseCase;
use crate::domain::entities::auth::{Login, LoginToken};
use crate::domain::repositories::auth::AuthRepositories;
use crate::shared::exceptions::custom_error::CustomError;
use crate::shared::middleware::jwt::create_token;

pub struct AuthUseCaseImpl<T: AuthRepositories> {
    repository: T,
    jwt_secret: String,
}

impl<T: AuthRepositories> AuthUseCaseImpl<T> {
    pub fn new(repository: T, jwt_secret: String) -> Self {
        Self { repository, jwt_secret }
    }
}

#[async_trait]
impl<T: AuthRepositories> AuthUseCase for AuthUseCaseImpl<T> {
    async fn login(&self, payload: Login) -> Result<LoginToken, CustomError> {
        let user = self.repository.user_exists(&payload.username).await?;
        let is_valid = verify(&payload.password, &user.password).map_err(|e| CustomError::BusinessError(format!("Password verification failed: {}", e)))?;

        if is_valid {
            let token = create_token(user.id, self.jwt_secret.as_str());

            Ok(LoginToken {
                token,
            })
        } else {
            Err(CustomError::Unauthorized("Invalid credentials".to_string()))
        }
    }
}
