use crate::internal::pkg::exceptions::custom_error::{CustomError};
use crate::internal::server::domain::repositories::health_check::HealthCheckRepositories;
use async_trait::async_trait;
use deadpool_postgres::{GenericClient, Pool};
use std::sync::Arc;

pub struct HealthCheckRepositoriesImpl {
    db_conn: Arc<Pool>,
}

impl HealthCheckRepositoriesImpl {
    pub fn new(db_conn: Arc<Pool>) -> Self {
        Self { db_conn }
    }
}

#[async_trait]
impl HealthCheckRepositories for HealthCheckRepositoriesImpl {
    async fn readiness(&self) -> Result<(), CustomError> {
        let client = self.db_conn.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;
        client
            .execute(
                "SELECT 1",
                &[],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        Ok(())
    }
}