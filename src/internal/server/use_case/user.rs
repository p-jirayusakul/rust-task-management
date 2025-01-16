use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::server::domain::entities::user::{
    Login,
    LoginToken,
};
use crate::internal::server::domain::repositories::user::UserRepositories;
use crate::internal::server::domain::use_case::user::UserUseCase;
use async_trait::async_trait;
use bcrypt::{verify};

pub struct UserUseCaseImpl<T: UserRepositories> {
    repository: T,
}

impl<T: UserRepositories> UserUseCaseImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: UserRepositories> UserUseCase for UserUseCaseImpl<T> {
    async fn login(&self, payload: Login) -> Result<LoginToken, CustomError>{

        let user = self.repository.user_exists(&payload.username).await?;
        let is_valid = verify(&payload.password, &user.password).map_err(|e| CustomError::BusinessError(format!("Password verification failed: {}", e)))?;

        if is_valid {
            Ok(LoginToken {
                token: "token".to_string(),
            })
        } else {
            Err(CustomError::Unauthorized("Invalid credentials".to_string()))
        }
    }
}
