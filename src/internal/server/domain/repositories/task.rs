use crate::internal::pkg::exceptions::custom_error::MyError;
use crate::internal::server::domain::entities::task::{CreateTask, Task, UpdateTask, UpdateTaskPriorityLevels, UpdateTaskStatus};
use async_trait::async_trait;

#[async_trait]
pub trait TaskRepositories: Send + Sync {
    async fn list_task(&self) -> Result<Vec<Task>, MyError>;
    async fn get_task(&self, id: i64) -> Result<Task, MyError>;
    async fn create_task(&self, task: CreateTask) -> Result<i64, MyError>;
    async fn update_task(&self, task: UpdateTask) -> Result<(), MyError>;
    async fn update_task_status(&self, task: UpdateTaskStatus) -> Result<(), MyError>;
    async fn update_task_priority_levels(&self, task: UpdateTaskPriorityLevels) -> Result<(), MyError>;
    async fn delete_task(&self, id: i64) -> Result<(), MyError>;
    async fn task_exists(&self, id: i64) -> Result<bool, MyError>;
    async fn task_status_exists(&self, id: i64) -> Result<bool, MyError>;
    async fn priority_exists(&self, id: i64) -> Result<bool, MyError>;
}
