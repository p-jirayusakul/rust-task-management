use async_trait::async_trait;
use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::server::domain::entities::task::{Task, CreateTask, UpdateTask, TaskID, UpdateTaskStatus, UpdateTaskPriorityLevels};


#[async_trait]
pub trait TaskUseCase: Send + Sync {
    async fn list_task(&self) -> Result<Vec<Task>, CustomError>;
    async fn get_task(&self, id: i64) -> Result<Task, CustomError>;
    async fn create_task(&self, task: CreateTask) -> Result<TaskID, CustomError>;
    async fn update_task(&self, task: UpdateTask) -> Result<(), CustomError>;
    async fn update_task_status(&self, task: UpdateTaskStatus) -> Result<(), CustomError>;
    async fn update_task_priority_levels(&self, task: UpdateTaskPriorityLevels) -> Result<(), CustomError>;
    async fn delete_task(&self, id: i64) -> Result<(), CustomError>;
}