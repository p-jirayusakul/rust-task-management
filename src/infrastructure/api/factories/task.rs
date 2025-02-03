use std::sync::Arc;
use actix_web::web;
use deadpool_postgres::Pool;
use crate::application::use_cases::task::TaskUseCaseImpl;
use crate::infrastructure::api::handlers::task::TaskHandler;
use crate::infrastructure::database::task::TaskRepositoriesImpl;
use crate::shared::utils::snowflake::SnowflakeImpl;

// ฟังก์ชันสำหรับสร้าง Task Handler
pub fn create_task_handler_data(
    pool: Arc<Pool>,
    snowflake_node: SnowflakeImpl,
) -> web::Data<TaskHandler<TaskUseCaseImpl<TaskRepositoriesImpl<SnowflakeImpl>>>> {
    let task_repository = TaskRepositoriesImpl::new(pool, snowflake_node);
    let task_use_case = TaskUseCaseImpl::new(task_repository);
    let task_handler = TaskHandler::new(task_use_case);
    web::Data::new(task_handler)
}