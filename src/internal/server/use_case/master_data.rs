use async_trait::async_trait;
use crate::internal::server::domain::entities::master_data::{
    MasterDataTaskStatus,
    MasterDataRole,
    MasterDataPriorityLevels,
};
use crate::internal::server::domain::repositories::master_data::MasterDataRepositories;
use crate::internal::server::domain::use_case::master_data::MasterDataUseCase;
use crate::internal::pkg::exceptions::custom_error::CustomError;

pub struct MasterDataUseCaseImpl<T: MasterDataRepositories> {
    repository: T,
}

impl<T: MasterDataRepositories> MasterDataUseCaseImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: MasterDataRepositories> MasterDataUseCase for MasterDataUseCaseImpl<T> {
    async fn list_task_status(&self) -> Result<Vec<MasterDataTaskStatus>, CustomError> {
        self.repository.list_task_status().await
    }
    async fn list_role(&self) -> Result<Vec<MasterDataRole>, CustomError> {
        self.repository.list_role().await
    }
    async fn list_priority_levels(&self) -> Result<Vec<MasterDataPriorityLevels>, CustomError> {
        self.repository.list_priority_levels().await
    }
}