use async_trait::async_trait;
use crate::shared::exceptions::custom_error::CustomError;

#[async_trait]
pub trait HealthCheckRepositories: Send + Sync {
    async fn readiness(&self) -> Result<(), CustomError>;
}