use async_trait::async_trait;
use crate::internal::pkg::exceptions::custom_error::MyError;
use crate::internal::server::domain::entities::task::{Task, CreateTask, UpdateTask, TaskID};


#[async_trait]
pub trait TaskUseCase {
    async fn list_task(&self) -> Result<Vec<Task>, MyError>;
    async fn get_task(&self, id: i64) -> Result<Task, MyError>;
    async fn create_task(&self, task: CreateTask) -> Result<TaskID, MyError>;
    async fn update_task(&self, task: UpdateTask) -> Result<(), MyError>;
    async fn delete_task(&self, id: i64) -> Result<(), MyError>;
}