use crate::internal::server::domain::entities::user::{
    Login,
    LoginToken,
};

use async_trait::async_trait;
use crate::internal::pkg::exceptions::custom_error::CustomError;

#[async_trait]
pub trait UserUseCase {
    async fn login(&self, payload: Login) -> Result<LoginToken, CustomError>;
}