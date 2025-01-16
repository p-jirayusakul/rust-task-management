use async_trait::async_trait;
use crate::internal::server::domain::entities::user::{
    User,
};
use crate::internal::pkg::exceptions::custom_error::CustomError;

#[async_trait]
pub trait UserRepositories: Send + Sync {
    async fn user_exists(&self, username: &str) -> Result<User, CustomError>;
}
