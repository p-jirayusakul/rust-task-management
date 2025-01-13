use crate::internal::pkg::exceptions::custom_error::{MyError, Status};
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
    async fn list_task(&self) -> Result<Vec<Task>, MyError> {
        self.repository.list_task().await
    }

    async fn get_task(&self, id: i64) -> Result<Task, MyError> {
        if !self.repository.task_exists(id).await? {
            return Err(MyError::new(Status::NotFound, format!("Task ID {} not found", id)));
        }
        self.repository.get_task(id).await
    }

    async fn create_task(&self, task: CreateTask) -> Result<TaskID, MyError> {
        if !self.repository.task_status_exists(task.task_status_id).await? {
            return Err(MyError::new(Status::NotFound, format!("Task Status ID {} not found", task.task_status_id)));
        }

        if !self.repository.priority_exists(task.priority_levels_id).await? {
            return Err(MyError::new(Status::NotFound, format!("Priority levels ID {} not found", task.priority_levels_id)));
        }

        self.repository.create_task(task).await.map(|id| TaskID { id })
    }

    async fn update_task(&self, task: UpdateTask) -> Result<(), MyError> {
        if !self.repository.task_exists(task.id).await? {
            return Err(MyError::new(Status::NotFound, format!("Task ID {} not found", task.id)));
        }

        if !self.repository.task_status_exists(task.task_status_id).await? {
            return Err(MyError::new(Status::NotFound, format!("Task Status ID {} not found", task.task_status_id)));
        }

        if !self.repository.priority_exists(task.priority_levels_id).await? {
            return Err(MyError::new(Status::NotFound, format!("Priority levels ID {} not found", task.priority_levels_id)));
        }

        self.repository.update_task(task).await
    }

    async fn update_task_status(&self, task: UpdateTaskStatus) -> Result<(), MyError> {
        if !self.repository.task_exists(task.id).await? {
            return Err(MyError::new(Status::NotFound, format!("Task ID {} not found", task.id)));
        }

        if !self.repository.task_status_exists(task.task_status_id).await? {
            return Err(MyError::new(Status::NotFound, format!("Task Status ID {} not found", task.task_status_id)));
        }

        self.repository.update_task_status(task).await
    }

    async fn update_task_priority_levels(&self, task: UpdateTaskPriorityLevels) -> Result<(), MyError> {
        if !self.repository.task_exists(task.id).await? {
            return Err(MyError::new(Status::NotFound, format!("Task ID {} not found", task.id)));
        }

        if !self.repository.priority_exists(task.priority_levels_id).await? {
            return Err(MyError::new(Status::NotFound, format!("Priority levels ID {} not found", task.priority_levels_id)));
        }

        self.repository.update_task_priority_levels(task).await
    }

    async fn delete_task(&self, id: i64) -> Result<(), MyError> {
        if !self.repository.task_exists(id).await? {
            return Err(MyError::new(Status::NotFound, format!("Task ID {} not found", id)));
        }
        self.repository.delete_task(id).await
    }
}