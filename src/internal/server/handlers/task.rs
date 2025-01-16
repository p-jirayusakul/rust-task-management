use crate::internal::pkg::middleware::response::{response_success};
use crate::internal::server::domain::entities::task::{CreateTask as CreateTaskEntity, UpdateTask as UpdateTaskEntity, UpdateTaskPriorityLevels as UpdateTaskPriorityLevelsEntity, UpdateTaskStatus as UpdateTaskStatusEntity};
use crate::internal::server::domain::use_case::task::TaskUseCase;
use crate::internal::server::request::task::{TaskRequest, UpdateTaskPriorityLevelsRequest, UpdateTaskStatusRequest};
use actix_web::{web, HttpResponse, Responder};
use actix_web::middleware::from_fn;
use validator::Validate;
use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::pkg::middleware::auth::jwt_middleware;

pub struct TaskHandler<T: TaskUseCase + Send + Sync> {
    use_case: T,
}

impl<T: TaskUseCase + Send + Sync> TaskHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    pub async fn list_task(handler: web::Data<TaskHandler<T>>) -> Result<impl Responder, CustomError> {
        match handler.use_case.list_task().await {
            Ok(tasks) => Ok(HttpResponse::Ok().json(response_success("get task completed", tasks))),
            Err(e) => Err(e),
        }
    }

    pub async fn get_task(handler: web::Data<TaskHandler<T>>, path: web::Path<i64>) -> Result<impl Responder, CustomError> {
        let task_id = path.into_inner();
        match handler.use_case.get_task(task_id).await {
            Ok(task) => Ok(HttpResponse::Ok().json(response_success("get task completed", task))),
            Err(e) => Err(e),
        }
    }

    pub async fn create_task(
        handler: web::Data<TaskHandler<T>>,
        body: web::Json<TaskRequest>,
    ) ->  Result<impl Responder, CustomError> {
        
        body.validate().map_err(|e| CustomError::ValidationError(e.to_string()))?;

        let new_task_entity = CreateTaskEntity {
            title: body.title.clone(),
            description: body.description.clone(),
            task_status_id: body.task_status_id,
            priority_levels_id: body.priority_levels_id,
            created_by: 1844995683120058368,
        };

        match handler.use_case.create_task(new_task_entity).await {
            Ok(task_id) => Ok(HttpResponse::Created().json(response_success("Task created successfully", task_id))),
            Err(e) => Err(e),
        }
    }

    pub async fn update_task(
        handler: web::Data<TaskHandler<T>>,
        body: web::Json<TaskRequest>,
        path: web::Path<i64>
    ) -> Result<impl Responder, CustomError> {
        let task_id = path.into_inner();

        body.validate().map_err(|e| CustomError::ValidationError(e.to_string()))?;

        let update_task_entity = UpdateTaskEntity {
            id: task_id,
            title: body.title.clone(),
            description: body.description.clone(),
            task_status_id: body.task_status_id,
            priority_levels_id: body.priority_levels_id,
            updated_by: 1844995683120058368,
        };

        match handler.use_case.update_task(update_task_entity).await {
            Ok(..) => Ok(HttpResponse::Ok().json(response_success("Task updated successfully", ()))),
            Err(e) => Err(e),
        }
    }

    pub async fn update_task_status(
        handler: web::Data<TaskHandler<T>>,
        body: web::Json<UpdateTaskStatusRequest>,
        path: web::Path<i64>
    ) -> Result<impl Responder, CustomError> {
        let task_id = path.into_inner();

        body.validate().map_err(|e| CustomError::ValidationError(e.to_string()))?;

        let update_task_entity = UpdateTaskStatusEntity {
            id: task_id,
            task_status_id: body.task_status_id,
            updated_by: 1844995683120058368,
        };

        match handler.use_case.update_task_status(update_task_entity).await {
            Ok(..) => Ok(HttpResponse::Ok().json(response_success("Task status updated successfully", ()))),
            Err(e) => Err(e),
        }
    }

    pub async fn update_task_priority_levels(
        handler: web::Data<TaskHandler<T>>,
        body: web::Json<UpdateTaskPriorityLevelsRequest>,
        path: web::Path<i64>
    ) -> Result<impl Responder, CustomError> {
        let task_id = path.into_inner();

        body.validate().map_err(|e| CustomError::ValidationError(e.to_string()))?;

        let update_task_entity = UpdateTaskPriorityLevelsEntity {
            id: task_id,
            priority_levels_id: body.priority_levels_id,
            updated_by: 1844995683120058368,
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


pub fn configure_routes<T: TaskUseCase + Send + Sync + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/task")
            .wrap(from_fn(jwt_middleware))
            .route("", web::get().to(TaskHandler::<T>::list_task))
            .route("/{task_id}", web::get().to(TaskHandler::<T>::get_task))
            .route("", web::post().to(TaskHandler::<T>::create_task))
            .route("/{task_id}", web::put().to(TaskHandler::<T>::update_task))
            .route("/{task_id}/task-status", web::patch().to(TaskHandler::<T>::update_task_status))
            .route("/{task_id}/priority-levels", web::patch().to(TaskHandler::<T>::update_task_priority_levels))
            .route("/{task_id}", web::delete().to(TaskHandler::<T>::delete_task))
        ,
    );
}