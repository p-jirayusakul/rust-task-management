use crate::internal::pkg::exceptions::custom_error::{CustomError};
use crate::internal::pkg::utils::snowflake::Snowflake;
use crate::internal::server::domain::entities::task::{CreateTask, Task, UpdateTask, UpdateTaskPriorityLevels, UpdateTaskStatus};
use crate::internal::server::domain::repositories::task::TaskRepositories;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
pub struct TaskImpl<S: Snowflake + Send + Sync> {
    pool: Arc<Pool>,
    snowflake_id: S,
}

impl<S: Snowflake + Send + Sync> TaskImpl<S> {
    pub fn new(pool: Arc<Pool>, snowflake_id: S) -> Self {
        Self { pool, snowflake_id }
    }
}

#[async_trait]
impl<S: Snowflake + Send + Sync> TaskRepositories for TaskImpl<S> {
    async fn list_task(&self) -> Result<Vec<Task>, CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        let rows = client
            .query(
                "SELECT id, title, description, task_status_id, priority_levels_id, created_by, created_at, updated_at, updated_by FROM public.task;",
                &[],
            )
            .await.map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        let tasks: Vec<Task> = rows
            .iter()
            .map(|row| Task {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                task_status_id: row.get("task_status_id"),
                priority_levels_id: row.get("priority_levels_id"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            })
            .collect();

        Ok(tasks)
    }
    async fn get_task(&self, id: i64) -> Result<Task, CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        let row = client
            .query_one(
                "SELECT id, title, description, task_status_id, priority_levels_id, created_by, created_at, updated_at, updated_by FROM public.task WHERE id = $1;",
                &[&id],
            )
            .await.map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        let task = Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            task_status_id: row.get("task_status_id"),
            priority_levels_id: row.get("priority_levels_id"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        };

        Ok(task)
    }
    async fn create_task(&self, task: CreateTask) -> Result<i64, CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;
        let new_id = self.snowflake_id.generate() as i64;

        let row = client
            .query_one(
                "INSERT INTO public.task (id, title, description, task_status_id, priority_levels_id, created_by, created_at) VALUES ($1, $2, $3, $4, $5, $6, NOW()) RETURNING id;",
                &[
                    &new_id,
                    &task.title,
                    &task.description,
                    &task.task_status_id,
                    &task.priority_levels_id,
                    &task.created_by,
                ],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        Ok(row.get(0))
    }

    async fn update_task(&self, task: UpdateTask) -> Result<(), CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        client
            .execute(
                "UPDATE public.task 
             SET title = $1, 
                 description = $2, 
                 task_status_id = $3, 
                 priority_levels_id = $4, 
                 updated_at = NOW(), 
                 updated_by = $5 
             WHERE id = $6;",
                &[
                    &task.title,
                    &task.description,
                    &task.task_status_id,
                    &task.priority_levels_id,
                    &task.updated_by,
                    &task.id,
                ],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        Ok(())
    }

    async fn update_task_status(&self, task: UpdateTaskStatus) -> Result<(), CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        client
            .execute(
                "UPDATE public.task
                 SET task_status_id = $1,
                     updated_at = NOW(),
                     updated_by = $2
                 WHERE id = $3;",
                &[
                    &task.task_status_id,
                    &task.updated_by,
                    &task.id,
                ],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        Ok(())
    }

    async fn update_task_priority_levels(&self, task: UpdateTaskPriorityLevels) -> Result<(), CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        client
            .execute(
                "UPDATE public.task
                 SET priority_levels_id = $1,
                     updated_at = NOW(),
                     updated_by = $2
                 WHERE id = $3;",
                &[
                    &task.priority_levels_id,
                    &task.updated_by,
                    &task.id,
                ],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        Ok(())
    }

    async fn delete_task(&self, id: i64) -> Result<(), CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        client
            .execute(
                "DELETE FROM public.task WHERE id = $1;",
                &[&id],
            )
            .await
            .map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        Ok(())
    }

    async fn task_exists(&self, id: i64) -> Result<bool, CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        let row = client
            .query_one(
                "SELECT (COUNT(id) > 0) as is_already_exists FROM public.task WHERE id = $1;",
                &[&id],
            )
            .await.map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        Ok(row.get::<_, bool>("is_already_exists"))
    }

    async fn task_status_exists(&self, id: i64) -> Result<bool, CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        let row = client
            .query_one(
                "SELECT (COUNT(id) > 0) as is_already_exists FROM public.master_data_task_status WHERE id = $1;",
                &[&id],
            )
            .await.map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        Ok(row.get::<_, bool>("is_already_exists"))
    }

    async fn priority_exists(&self, id: i64) -> Result<bool, CustomError> {
        let client = self.pool.get().await.map_err(|e| CustomError::RepositoryError(format!("Failed to get database connection: {}", e)))?;

        let row = client
            .query_one(
                "SELECT (COUNT(id) > 0) as is_already_exists FROM public.master_data_priority_levels WHERE id = $1;",
                &[&id],
            )
            .await.map_err(|e| CustomError::RepositoryError(format!("Database query failed: {}", e)))?;

        Ok(row.get::<_, bool>("is_already_exists"))
    }
}