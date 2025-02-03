use actix_web::web;
use crate::application::interfaces::master_data::MasterDataUseCase;
use crate::infrastructure::api::handlers::master_data_handler::MasterDataHandler;

pub fn configure_master_data_routes<T: MasterDataUseCase + Send + Sync + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/master-data")
            .route("/task-status", web::get().to(MasterDataHandler::<T>::list_task_status))
            .route("/role", web::get().to(MasterDataHandler::<T>::list_role))
            .route("/priority-levels", web::get().to(MasterDataHandler::<T>::list_priority_levels))
    );
}