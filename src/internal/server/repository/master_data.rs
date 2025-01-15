use crate::internal::pkg::exceptions::custom_error::{CustomError};
use crate::internal::server::domain::entities::master_data::{
    MasterDataPriorityLevels,
    MasterDataRole,
    MasterDataTaskStatus,
};
use crate::internal::server::domain::repositories::master_data::MasterDataRepositories;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;

pub struct MasterDataImpl {
    pool: Arc<Pool>,
}

impl MasterDataImpl {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MasterDataRepositories for MasterDataImpl {
    async fn list_task_status(&self) -> Result<Vec<MasterDataTaskStatus>, CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        let rows = client
            .query(
                "SELECT id, title, code, active, created_by, created_at, updated_at, updated_by FROM public.master_data_task_status;",
                &[],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        let statuses: Vec<MasterDataTaskStatus> = rows
            .iter()
            .map(|row| MasterDataTaskStatus {
                id: row.get("id"),
                title: row.get("title"),
                code: row.get("code"),
                active: row.get("active"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            })
            .collect();

        Ok(statuses)
    }

    async fn list_role(&self) -> Result<Vec<MasterDataRole>, CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;
        let rows = client
            .query(
                "SELECT id, title, code, active, created_by, created_at, updated_at, updated_by FROM public.master_data_role;",
                &[],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        let statuses: Vec<MasterDataRole> = rows
            .iter()
            .map(|row| MasterDataRole {
                id: row.get("id"),
                title: row.get("title"),
                code: row.get("code"),
                active: row.get("active"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            })
            .collect();

        Ok(statuses)
    }

    async fn list_priority_levels(&self) -> Result<Vec<MasterDataPriorityLevels>, CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;
        let rows = client
            .query(
                "SELECT id, seq, title, code, active, created_by, created_at, updated_at, updated_by FROM public.master_data_priority_levels;",
                &[],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        let statuses: Vec<MasterDataPriorityLevels> = rows
            .iter()
            .map(|row| MasterDataPriorityLevels {
                id: row.get("id"),
                seq: row.get("seq"),
                title: row.get("title"),
                code: row.get("code"),
                active: row.get("active"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            })
            .collect();

        Ok(statuses)
    }
}