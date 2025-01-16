use crate::internal::pkg::exceptions::custom_error::{CustomError};

use crate::internal::server::domain::repositories::user::UserRepositories;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use crate::internal::pkg::exceptions::error_message::{RECORD_NOT_FOUND, USERNAME_NOT_FOUND};
use crate::internal::server::domain::entities::user::User;

pub struct UserRepositoriesImpl {
    pool: Arc<Pool>,
}

impl UserRepositoriesImpl {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositories for UserRepositoriesImpl {
    async fn user_exists(&self, username: &str) -> Result<User, CustomError>{
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        let row = client
            .query_one(
                "SELECT id, username, password FROM public.users WHERE username = $1 LIMIT 1;",
                &[&username],
            )
            .await.map_err(|e| {
            if e.to_string().contains(RECORD_NOT_FOUND) {
                return CustomError::Unauthorized(format!("{}: {}", USERNAME_NOT_FOUND, username));
            }
            CustomError::RepositoryError(format!("Database query failed: {}", e))
        })?;

        let user = User {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
        };

        Ok(user)
    }
}