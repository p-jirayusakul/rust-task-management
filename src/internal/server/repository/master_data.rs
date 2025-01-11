use std::sync::Arc;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use crate::internal::server::domain::entities::master_data::{
    MasterDataTaskStatus,
    MasterDataRole,
    MasterDataPriorityLevels,
};
use crate::internal::server::domain::repositories::master_data::MasterDataRepositories;
use crate::internal::pkg::exceptions::custom_error::MyError;

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
    async fn list_task_status(&self) -> Result<Vec<MasterDataTaskStatus>, MyError> {
        let client = self.pool.get().await.map_err(|e| MyError::new(format!("Failed to get database connection: {}", e)))?;

        let rows = client
            .query(
                "SELECT id, title, code, active, created_by, created_at, updated_at, updated_by FROM public.master_data_task_status;",
                &[],
            )
            .await.map_err(|e| MyError::new(format!("Database query failed: {}", e)))?;

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

    async fn list_role(&self) -> Result<Vec<MasterDataRole>, MyError> {
        let client = self.pool.get().await.map_err(|e| MyError::new(format!("Failed to get database connection: {}", e)))?;
        let rows = client
            .query(
                "SELECT id, title, code, active, created_by, created_at, updated_at, updated_by FROM public.master_data_role;",
                &[],
            )
            .await.map_err(|e| MyError::new(format!("Database query failed: {}", e)))?;

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

    async fn list_priority_levels(&self) -> Result<Vec<MasterDataPriorityLevels>, MyError> {
        let client = self.pool.get().await.map_err(|e| MyError::new(format!("Failed to get database connection: {}", e)))?;
        let rows = client
            .query(
                "SELECT id, seq, title, code, active, created_by, created_at, updated_at, updated_by FROM public.master_data_priority_levels;",
                &[],
            )
            .await.map_err(|e| MyError::new(format!("Database query failed: {}", e)))?;

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