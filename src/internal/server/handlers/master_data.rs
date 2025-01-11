use crate::internal::server::domain::use_case::master_data::MasterDataUseCase;
use crate::internal::pkg::middleware::response::{
    response_success,
    response_error,
};
use actix_web::{ web, HttpResponse, Responder};

#[derive(Clone)]
pub struct MasterDataHandler<T: MasterDataUseCase + Send + Sync> {
    use_case: T,
}

impl<T: MasterDataUseCase + Send + Sync> MasterDataHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    pub async fn list_task_status(
        handler: web::Data<MasterDataHandler<T>>,
    ) -> impl Responder {
        match handler.use_case.list_task_status().await {
            Ok(task_statuses) => HttpResponse::Ok().json(response_success("get list task status completed", task_statuses)),
            Err(e) => HttpResponse::InternalServerError().json(response_error(&e.message)),
        }
    }
}

// Static route registration
pub fn configure_routes<T: MasterDataUseCase + Send + Sync + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/master-data")
            .route("/task-status", web::get().to(MasterDataHandler::<T>::list_task_status))
    );
}