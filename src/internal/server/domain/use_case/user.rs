use crate::internal::server::domain::entities::user::{
    Login,
    LoginToken,
};

use crate::internal::pkg::exceptions::custom_error::CustomError;
use async_trait::async_trait;

#[async_trait]
pub trait UserUseCase {
    async fn login(&self, payload: Login) -> Result<LoginToken, CustomError>;
}