use actix_web::web;
use crate::application::interfaces::task::TaskUseCase;
use crate::infrastructure::api::handlers::task::TaskHandler;
use crate::shared::middleware::auth::JwtMiddleware;

pub fn configure_task_routes<T: TaskUseCase + Send + Sync + 'static>(cfg: &mut web::ServiceConfig, jwt_secret: String) {
    cfg.service(
        web::scope("/task")
            .wrap(JwtMiddleware::new(jwt_secret))
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