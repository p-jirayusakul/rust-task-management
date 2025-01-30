use crate::internal::pkg::exceptions::custom_error::CustomError;
use crate::internal::pkg::middleware::response::response_success;
use crate::internal::server::domain::use_case::master_data::MasterDataUseCase;
use actix_web::{web, HttpResponse, Responder};

pub struct MasterDataHandler<T: MasterDataUseCase + Send + Sync> {
    use_case: T,
}

impl<T: MasterDataUseCase + Send + Sync> MasterDataHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    async fn list_task_status(
        handler: web::Data<MasterDataHandler<T>>,
    ) -> Result<impl Responder, CustomError> {
        // เรีนยก use case และ return
        match handler.use_case.list_task_status().await {
            Ok(items) => Ok(HttpResponse::Ok().json(response_success("get list task status completed", items))),
            Err(e) => Err(e)
        }
    }

    async fn list_role(
        handler: web::Data<MasterDataHandler<T>>,
    ) -> Result<impl Responder, CustomError> {
        match handler.use_case.list_role().await {
            Ok(items) => Ok(HttpResponse::Ok().json(response_success("get list role completed", items))),
            Err(e) => Err(e)
        }
    }

    async fn list_priority_levels(
        handler: web::Data<MasterDataHandler<T>>,
    ) -> Result<impl Responder, CustomError> {
        match handler.use_case.list_priority_levels().await {
            Ok(items) => Ok(HttpResponse::Ok().json(response_success("get list priority levels completed", items))),
            Err(e) => Err(e)
        }
    }
}

pub fn configure_routes<T: MasterDataUseCase + Send + Sync + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/master-data")
            .route("/task-status", web::get().to(MasterDataHandler::<T>::list_task_status))
            .route("/role", web::get().to(MasterDataHandler::<T>::list_role))
            .route("/priority-levels", web::get().to(MasterDataHandler::<T>::list_priority_levels))
    );
}