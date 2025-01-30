use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::server::domain::entities::master_data::{
    MasterDataPriorityLevels,
    MasterDataRole,
    MasterDataTaskStatus,
};
use crate::internal::server::domain::repositories::master_data::MasterDataRepositories;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;

pub struct MasterDataRepositoriesImpl {
    db_conn: Arc<Pool>,
}

impl MasterDataRepositoriesImpl {
    pub fn new(db_conn: Arc<Pool>) -> Self {
        Self { db_conn }
    }
}

#[async_trait]
impl MasterDataRepositories for MasterDataRepositoriesImpl {
    async fn list_task_status(&self) -> Result<Vec<MasterDataTaskStatus>, CustomError> {
        let client = self.db_conn.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        let rows = client
            .query(
                "SELECT id, title, code FROM public.master_data_task_status WHERE active IS TRUE;",
                &[],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        let items: Vec<MasterDataTaskStatus> = rows
            .iter()
            .map(|row| MasterDataTaskStatus {
                id: row.get("id"),
                title: row.get("title"),
                code: row.get("code"),
            })
            .collect();

        Ok(items)
    }

    async fn list_role(&self) -> Result<Vec<MasterDataRole>, CustomError> {
        let client = self.db_conn.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;
        let rows = client
            .query(
                "SELECT id, title, code FROM public.master_data_role WHERE active IS TRUE;",
                &[],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        let items: Vec<MasterDataRole> = rows
            .iter()
            .map(|row| MasterDataRole {
                id: row.get("id"),
                title: row.get("title"),
                code: row.get("code"),
            })
            .collect();

        Ok(items)
    }

    async fn list_priority_levels(&self) -> Result<Vec<MasterDataPriorityLevels>, CustomError> {
        let client = self.db_conn.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;
        let rows = client
            .query(
                "SELECT id, title, code FROM public.master_data_priority_levels WHERE active IS TRUE ORDER BY seq ASC;",
                &[],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        let items: Vec<MasterDataPriorityLevels> = rows
            .iter()
            .map(|row| MasterDataPriorityLevels {
                id: row.get("id"),
                title: row.get("title"),
                code: row.get("code"),
            })
            .collect();

        Ok(items)
    }
}