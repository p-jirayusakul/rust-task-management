use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::server::domain::entities::user::{
    Login,
    LoginToken,
};
use crate::internal::server::domain::repositories::user::UserRepositories;
use crate::internal::server::domain::use_case::user::UserUseCase;
use async_trait::async_trait;
use bcrypt::{verify};
use crate::internal::pkg::middleware::jwt::create_token;

pub struct UserUseCaseImpl<T: UserRepositories> {
    repository: T,
    jwt_secret: String
}

impl<T: UserRepositories> UserUseCaseImpl<T> {
    pub fn new(repository: T, jwt_secret: String) -> Self {
        Self { repository, jwt_secret }
    }
}

#[async_trait]
impl<T: UserRepositories> UserUseCase for UserUseCaseImpl<T> {
    async fn login(&self, payload: Login) -> Result<LoginToken, CustomError>{

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
