use async_trait::async_trait;
use crate::internal::server::domain::repositories::health_check::HealthCheckRepositories;
use crate::internal::server::domain::use_case::health_check::HealthCheckUseCase;
use crate::internal::pkg::exceptions::custom_error::CustomError;

pub struct HealthCheckUseCaseImpl<T: HealthCheckRepositories> {
    repository: T,
}

impl<T: HealthCheckRepositories> HealthCheckUseCaseImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: HealthCheckRepositories> HealthCheckUseCase for HealthCheckUseCaseImpl<T> {
    async fn readiness(&self) -> Result<(), CustomError> {
        self.repository.readiness().await
    }
}