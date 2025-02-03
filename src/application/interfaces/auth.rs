use async_trait::async_trait;
use crate::domain::entities::auth::{Login, LoginToken};
use crate::shared::exceptions::custom_error::CustomError;

#[async_trait]
pub trait AuthUseCase {
    async fn login(&self, payload: Login) -> Result<LoginToken, CustomError>;
}