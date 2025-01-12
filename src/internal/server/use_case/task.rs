use crate::internal::pkg::exceptions::custom_error::MyError;
use crate::internal::server::domain::entities::task::{CreateTask, Task, TaskID, UpdateTask};
use crate::internal::server::domain::repositories::task::TaskRepositories;
use crate::internal::server::domain::use_case::task::TaskUseCase;
use async_trait::async_trait;

pub struct TaskUseCaseImpl<T: TaskRepositories> {
    repository: T,
}

impl<T: TaskRepositories> TaskUseCaseImpl<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: TaskRepositories> TaskUseCase for TaskUseCaseImpl<T> {
    async fn list_task(&self) -> Result<Vec<Task>, MyError> {
        self.repository.list_task().await
    }

    async fn get_task(&self, id: i64) -> Result<Task, MyError> {
        self.repository.get_task(id).await
    }

    async fn create_task(&self, task: CreateTask) -> Result<TaskID, MyError> {

        if !self.repository.is_task_status_already_exist(task.task_status_id).await? {
            return Err(MyError { message: "Task status not found".to_string() });
        }

        if !self.repository.is_priority_levels_already_exist(task.priority_levels_id).await? {
            return Err(MyError { message: "Priority levels not found".to_string() });
        }
        
        self.repository.create_task(task).await.map(|id| TaskID { id })
    }

    async fn update_task(&self, task: UpdateTask) -> Result<(), MyError> {
        self.repository.update_task(task).await
    }

    async fn delete_task(&self, id: i64) -> Result<(), MyError> {
        self.repository.delete_task(id).await
    }
}