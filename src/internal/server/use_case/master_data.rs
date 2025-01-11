use async_trait::async_trait;
use crate::internal::server::domain::entities::master_data::{
    MasterDataTaskStatus,
    MasterDataRole,
    MasterDataPriorityLevels,
};
use crate::internal::server::domain::repositories::master_data::MasterDataRepositories;
use crate::internal::server::domain::use_case::master_data::MasterDataUseCase;
use crate::internal::pkg::exceptions::custom_error::MyError;

#[derive(Clone)] // ทำให้สามารถ Clone ได้
pub struct MasterDataUseCaseImpl<T: MasterDataRepositories + Clone> {
    repository: T,
}

impl<T: MasterDataRepositories + Clone> MasterDataUseCaseImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: MasterDataRepositories + Clone> MasterDataUseCase for MasterDataUseCaseImpl<T> {
    async fn list_task_status(&self) -> Result<Vec<MasterDataTaskStatus>, MyError> {
        self.repository.list_task_status().await
    }
    async fn list_role(&self) -> Result<Vec<MasterDataRole>, MyError> {
        self.repository.list_role().await
    }
    async fn list_priority_levels(&self) -> Result<Vec<MasterDataPriorityLevels>, MyError> {
        self.repository.list_priority_levels().await
    }
}