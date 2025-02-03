
use actix_web::{web, HttpResponse, Responder};
use crate::application::interfaces::master_data::MasterDataUseCase;
use crate::shared::exceptions::custom_error::CustomError;
use crate::shared::middleware::response::response_success;

pub struct MasterDataHandler<T: MasterDataUseCase + Send + Sync> {
    use_case: T,
}

impl<T: MasterDataUseCase + Send + Sync> MasterDataHandler<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }

    pub(crate) async fn list_task_status(
        handler: web::Data<MasterDataHandler<T>>,
    ) -> Result<impl Responder, CustomError> {
        // เรีนยก use case และ return
        match handler.use_case.list_task_status().await {
            Ok(items) => Ok(HttpResponse::Ok().json(response_success("get list task status successfully", items))),
            Err(e) => Err(e)
        }
    }

    pub(crate) async fn list_role(
        handler: web::Data<MasterDataHandler<T>>,
    ) -> Result<impl Responder, CustomError> {
        match handler.use_case.list_role().await {
            Ok(items) => Ok(HttpResponse::Ok().json(response_success("get list role successfully", items))),
            Err(e) => Err(e)
        }
    }

    pub(crate) async fn list_priority_levels(
        handler: web::Data<MasterDataHandler<T>>,
    ) -> Result<impl Responder, CustomError> {
        match handler.use_case.list_priority_levels().await {
            Ok(items) => Ok(HttpResponse::Ok().json(response_success("get list priority levels successfully", items))),
            Err(e) => Err(e)
        }
    }
}