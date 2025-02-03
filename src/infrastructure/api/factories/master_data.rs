use std::sync::Arc;
use actix_web::web;
use deadpool_postgres::Pool;
use crate::application::use_cases::master_data::MasterDataUseCaseImpl;
use crate::infrastructure::api::handlers::master_data_handler::MasterDataHandler;
use crate::infrastructure::database::master_data::MasterDataRepositoriesImpl;

pub fn create_master_data_handler_data(
    pool: Arc<Pool>,
) -> web::Data<MasterDataHandler<MasterDataUseCaseImpl<MasterDataRepositoriesImpl>>> {
    let master_data_repository = MasterDataRepositoriesImpl::new(pool);
    let master_data_use_case = MasterDataUseCaseImpl::new(master_data_repository);
    let master_data_handler = MasterDataHandler::new(master_data_use_case);
    web::Data::new(master_data_handler)
}
