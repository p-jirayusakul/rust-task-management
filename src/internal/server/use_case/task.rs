use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::pkg::exceptions::error_message::{TASK_NOT_FOUND};
use crate::internal::server::domain::entities::task::{CreateTask, Task, TaskID, UpdateTask, UpdateTaskPriorityLevels, UpdateTaskStatus};
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
    async fn list_task(&self) -> Result<Vec<Task>, CustomError> {
        self.repository.list_task().await
    }

    async fn get_task(&self, id: i64) -> Result<Task, CustomError> {
        self.repository.get_task(id).await
    }

    async fn create_task(&self, task: CreateTask) -> Result<TaskID, CustomError> {
        self.repository.create_task(task).await.map(|id| TaskID { id })
    }

    async fn update_task(&self, task: UpdateTask) -> Result<(), CustomError> {
        if !self.repository.task_exists(task.id).await? {
            return Err(not_found_error(TASK_NOT_FOUND, task.id))
        }

        self.repository.update_task(task).await
    }

    async fn update_task_status(&self, task: UpdateTaskStatus) -> Result<(), CustomError> {
        if !self.repository.task_exists(task.id).await? {
            return Err(not_found_error(TASK_NOT_FOUND, task.id))
        }

        self.repository.update_task_status(task).await
    }

    async fn update_task_priority_levels(&self, task: UpdateTaskPriorityLevels) -> Result<(), CustomError> {
        if !self.repository.task_exists(task.id).await? {
            return Err(not_found_error(TASK_NOT_FOUND, task.id))
        }

        self.repository.update_task_priority_levels(task).await
    }

    async fn delete_task(&self, id: i64) -> Result<(), CustomError> {
        if !self.repository.task_exists(id).await? {
            return Err(not_found_error(TASK_NOT_FOUND, id))
        }
        self.repository.delete_task(id).await
    }
}

fn not_found_error(item: &str, id: i64) -> CustomError {
    CustomError::NotFound(format!("{}: {}", item, id))
}