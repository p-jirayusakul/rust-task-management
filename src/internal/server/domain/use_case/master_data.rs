use crate::internal::server::domain::entities::master_data::{
    MasterDataTaskStatus,
    MasterDataRole,
    MasterDataPriorityLevels,
};

use async_trait::async_trait;
use crate::internal::pkg::exceptions::custom_error::CustomError;

#[async_trait]
pub trait MasterDataUseCase {
    async fn list_task_status(&self) -> Result<Vec<MasterDataTaskStatus>, CustomError>;
    async fn list_role(&self) -> Result<Vec<MasterDataRole>, CustomError>;
    async fn list_priority_levels(&self) -> Result<Vec<MasterDataPriorityLevels>, CustomError>;
}