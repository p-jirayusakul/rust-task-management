use crate::internal::server::domain::entities::master_data::{
    MasterDataPriorityLevels,
    MasterDataRole,
    MasterDataTaskStatus,
};

use crate::internal::pkg::exceptions::custom_error::CustomError;
use async_trait::async_trait;

#[async_trait]
pub trait MasterDataUseCase: Send + Sync {
    async fn list_task_status(&self) -> Result<Vec<MasterDataTaskStatus>, CustomError>;
    async fn list_role(&self) -> Result<Vec<MasterDataRole>, CustomError>;
    async fn list_priority_levels(&self) -> Result<Vec<MasterDataPriorityLevels>, CustomError>;
}