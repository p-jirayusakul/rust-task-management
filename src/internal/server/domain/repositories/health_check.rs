use async_trait::async_trait;
use crate::internal::pkg::exceptions::custom_error::CustomError;

#[async_trait]
pub trait HealthCheckRepositories: Send + Sync {
    async fn readiness(&self) -> Result<(), CustomError>;
}