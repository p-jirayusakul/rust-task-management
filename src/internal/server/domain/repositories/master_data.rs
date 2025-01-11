use async_trait::async_trait;
use crate::internal::server::domain::entities::master_data::{
    MasterDataTaskStatus,
    MasterDataRole,
    MasterDataPriorityLevels,
};
use crate::internal::pkg::exceptions::custom_error::MyError;

#[async_trait]
pub trait MasterDataRepositories: Send + Sync {
    async fn list_task_status(&self) -> Result<Vec<MasterDataTaskStatus>, MyError>;
    async fn list_role(&self) -> Result<Vec<MasterDataRole>, MyError>;
    async fn list_priority_levels(&self) -> Result<Vec<MasterDataPriorityLevels>, MyError>;
}
