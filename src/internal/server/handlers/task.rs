use crate::internal::pkg::middleware::response::{
    response_error,
    response_success,
};
use crate::internal::server::domain::entities::task::CreateTask as CreateTaskEntity;
use crate::internal::server::domain::use_case::task::TaskUseCase;
use crate::internal::server::request::task::CreateTask;
use actix_web::{web, HttpResponse, Responder};
use validator::Validate;

pub struct TaskHandler<T: TaskUseCase + Send + Sync> {
    use_case: T,
}

impl<T: TaskUseCase + Send + Sync> TaskHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    pub async fn list_task(
        handler: web::Data<TaskHandler<T>>,
    ) -> impl Responder {
        match handler.use_case.list_task().await {
            Ok(tasks) => HttpResponse::Ok().json(response_success("get list task completed", tasks)),
            Err(e) => HttpResponse::build(e.http_status_code()).json(response_error(&e.message)),
        }
    }

    pub async fn create_task(
        handler: web::Data<TaskHandler<T>>,
        body: web::Json<CreateTask>,
    ) -> impl Responder {
        
        match body.validate() {
            Ok(_) => (),
            Err(e) => return HttpResponse::BadRequest().json(response_error(&e.to_string())),
        }

        let task: CreateTaskEntity = CreateTaskEntity {
            title: body.title.clone(),
            description: body.description.clone(),
            task_status_id: body.task_status_id,
            priority_levels_id: body.priority_levels_id,
            created_by: 1844995683120058368,
        };

        match handler.use_case.create_task(task).await {
            Ok(task_id) => HttpResponse::Created().json(response_success("Task created successfully", task_id)),
            Err(e) => HttpResponse::build(e.http_status_code()).json(response_error(&e.message))
        }
    }
}

// Static route registration
pub fn configure_routes<T: TaskUseCase + Send + Sync + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/task")
            .route("", web::get().to(TaskHandler::<T>::list_task))
            .route("", web::post().to(TaskHandler::<T>::create_task))
    );
}