use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::server::domain::entities::master_data::{
    MasterDataPriorityLevels,
    MasterDataRole,
    MasterDataTaskStatus,
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait MasterDataRepositories: Send + Sync {
    async fn list_task_status(&self) -> Result<Vec<MasterDataTaskStatus>, CustomError>;
    async fn list_role(&self) -> Result<Vec<MasterDataRole>, CustomError>;
    async fn list_priority_levels(&self) -> Result<Vec<MasterDataPriorityLevels>, CustomError>;
}
