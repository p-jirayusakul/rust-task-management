
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use validator::Validate;
use crate::application::interfaces::task::TaskUseCase;
use crate::domain::entities::task::{
    TaskCreateEntity,
    UpdateTask as UpdateTaskEntity,
    UpdateTaskStatus as UpdateTaskStatusEntity,
    UpdateTaskPriorityLevels as UpdateTaskPriorityLevelsEntity
};
use crate::infrastructure::api::requests::task::{TaskRequest, UpdateTaskPriorityLevelsRequest, UpdateTaskStatusRequest};
use crate::shared::exceptions::custom_error::CustomError;
use crate::shared::middleware::jwt::extract_user_id;
use crate::shared::middleware::response::response_success;

pub struct TaskHandler<T: TaskUseCase + Send + Sync> {
    use_case: T,
}

impl<T: TaskUseCase + Send + Sync> TaskHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    pub async fn list_task(handler: web::Data<TaskHandler<T>>) -> Result<impl Responder, CustomError> {
        match handler.use_case.list_task().await {
            Ok(tasks) => Ok(HttpResponse::Ok().json(response_success("get task successfully", tasks))),
            Err(e) => Err(e),
        }
    }

    pub async fn get_task(handler: web::Data<TaskHandler<T>>, path: web::Path<i64>) -> Result<impl Responder, CustomError> {
        let task_id = path.into_inner();
        match handler.use_case.get_task(task_id).await {
            Ok(task) => Ok(HttpResponse::Ok().json(response_success("get task successfully", task))),
            Err(e) => Err(e),
        }
    }

    pub async fn create_task(
        handler: web::Data<TaskHandler<T>>,
        body: web::Json<TaskRequest>,
        req: HttpRequest,
    ) -> Result<impl Responder, CustomError> {
        // user_id จาก JWT ที่ถอดมาจาก middleware
        let user_id = extract_user_id(&req).await?;

        // validate body request
        body.validate().map_err(|e| CustomError::ValidationError(e.to_string()))?;

        // เตรียมข้อมูลส่งให้ layer use case
        let new_task_entity = TaskCreateEntity {
            title: body.title.clone(),
            description: body.description.clone(),
            task_status_id: body.task_status_id,
            priority_levels_id: body.priority_levels_id,
            created_by: user_id,
        };

        // เรียก use case และ return
        match handler.use_case.create_task(new_task_entity).await {
            Ok(task_id) => Ok(HttpResponse::Created().json(response_success("Task created successfully", task_id))),
            Err(e) => Err(e),
        }
    }

    pub async fn update_task(
        handler: web::Data<TaskHandler<T>>,
        body: web::Json<TaskRequest>,
        path: web::Path<i64>,
        req: HttpRequest,
    ) -> Result<impl Responder, CustomError> {
        let user_id = extract_user_id(&req).await?;
        let task_id = path.into_inner();

        body.validate().map_err(|e| CustomError::ValidationError(e.to_string()))?;

        let update_task_entity = UpdateTaskEntity {
            id: task_id,
            title: body.title.clone(),
            description: body.description.clone(),
            task_status_id: body.task_status_id,
            priority_levels_id: body.priority_levels_id,
            updated_by: user_id,
        };

        match handler.use_case.update_task(update_task_entity).await {
            Ok(..) => Ok(HttpResponse::Ok().json(response_success("Task updated successfully", ()))),
            Err(e) => Err(e),
        }
    }

    pub async fn update_task_status(
        handler: web::Data<TaskHandler<T>>,
        body: web::Json<UpdateTaskStatusRequest>,
        path: web::Path<i64>,
        req: HttpRequest,
    ) -> Result<impl Responder, CustomError> {
        let user_id = extract_user_id(&req).await?;
        let task_id = path.into_inner();

        body.validate().map_err(|e| CustomError::ValidationError(e.to_string()))?;

        let update_task_entity = UpdateTaskStatusEntity {
            id: task_id,
            task_status_id: body.task_status_id,
            updated_by: user_id,
        };

        match handler.use_case.update_task_status(update_task_entity).await {
            Ok(..) => Ok(HttpResponse::Ok().json(response_success("Task status updated successfully", ()))),
            Err(e) => Err(e),
        }
    }

    pub async fn update_task_priority_levels(
        handler: web::Data<TaskHandler<T>>,
        body: web::Json<UpdateTaskPriorityLevelsRequest>,
        path: web::Path<i64>,
        req: HttpRequest,
    ) -> Result<impl Responder, CustomError> {
        let user_id = extract_user_id(&req).await?;
        let task_id = path.into_inner();

        body.validate().map_err(|e| CustomError::ValidationError(e.to_string()))?;

        let update_task_entity = UpdateTaskPriorityLevelsEntity {
            id: task_id,
            priority_levels_id: body.priority_levels_id,
            updated_by: user_id,
        };

        match handler.use_case.update_task_priority_levels(update_task_entity).await {
            Ok(..) => Ok(HttpResponse::Ok().json(response_success("Task priority levels updated successfully", ()))),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_task(handler: web::Data<TaskHandler<T>>, path: web::Path<i64>) -> Result<impl Responder, CustomError> {
        let task_id = path.into_inner();
        match handler.use_case.delete_task(task_id).await {
            Ok(..) => Ok(HttpResponse::Ok().json(response_success("Task deleted successfully", ()))),
            Err(e) => Err(e),
        }
    }
}