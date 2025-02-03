use async_trait::async_trait;
use crate::domain::entities::auth::User;
use crate::shared::exceptions::custom_error::CustomError;

#[async_trait]
pub trait AuthRepositories: Send + Sync {
    async fn user_exists(&self, username: &str) -> Result<User, CustomError>;
}
