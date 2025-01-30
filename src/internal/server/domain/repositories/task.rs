use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::server::domain::entities::task::{Task, TaskCreateEntity, UpdateTask, UpdateTaskPriorityLevels, UpdateTaskStatus};
use async_trait::async_trait;

use mockall::automock;

#[automock]
#[async_trait]
pub trait TaskRepositories: Send + Sync {
    async fn list_task(&self) -> Result<Vec<Task>, CustomError>;
    async fn get_task(&self, id: i64) -> Result<Task, CustomError>;
    async fn create_task(&self, task: TaskCreateEntity) -> Result<i64, CustomError>;
    async fn update_task(&self, task: UpdateTask) -> Result<(), CustomError>;
    async fn update_task_status(&self, task: UpdateTaskStatus) -> Result<(), CustomError>;
    async fn update_task_priority_levels(&self, task: UpdateTaskPriorityLevels) -> Result<(), CustomError>;
    async fn delete_task(&self, id: i64) -> Result<(), CustomError>;
    async fn task_exists(&self, id: i64) -> Result<bool, CustomError>;
}
