use async_trait::async_trait;
use crate::domain::entities::task::{Task, TaskCreateEntity, TaskID, UpdateTask, UpdateTaskPriorityLevels, UpdateTaskStatus};
use crate::shared::exceptions::custom_error::CustomError;

#[async_trait]
pub trait TaskUseCase: Send + Sync {
    async fn list_task(&self) -> Result<Vec<Task>, CustomError>;
    async fn get_task(&self, id: i64) -> Result<Task, CustomError>;
    async fn create_task(&self, task: TaskCreateEntity) -> Result<TaskID, CustomError>;
    async fn update_task(&self, task: UpdateTask) -> Result<(), CustomError>;
    async fn update_task_status(&self, task: UpdateTaskStatus) -> Result<(), CustomError>;
    async fn update_task_priority_levels(&self, task: UpdateTaskPriorityLevels) -> Result<(), CustomError>;
    async fn delete_task(&self, id: i64) -> Result<(), CustomError>;
}