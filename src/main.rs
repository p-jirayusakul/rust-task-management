use actix_web::{web, App, HttpServer};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use sonyflake::Sonyflake;
use std::env;
use std::sync::Arc;
use tokio_postgres::NoTls;

mod internal;

use internal::{
    pkg::utils::snowflake::SnowflakeImpl,
    server::{
        handlers::{
            master_data::{configure_routes as configure_master_data_routes, MasterDataHandler},
            task::{configure_routes as configure_task_routes, TaskHandler},
        },
        repository::{master_data::MasterDataImpl, task::TaskImpl},
        use_case::{master_data::MasterDataUseCaseImpl, task::TaskUseCaseImpl},
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up config
    if dotenv::from_filename(".env.local").is_err() {
        println!("Warning: .env.local not found. Using OS environment variables instead.");
    }

    let port_string = env::var("APP_PORT").expect("APP_PORT must be set in environment variables");
    let port: u16 = port_string.parse().expect("Failed to parse port as u16");
    let database_host = env::var("DB_HOST").expect("DB_HOST must be set in environment variables");
    let database_port = env::var("DB_PORT").expect("DB_PORT must be set in environment variables");
    let database_name =
        env::var("DB_DATABASE").expect("DB_DATABASE must be set in environment variables");
    let database_user =
        env::var("DB_USERNAME").expect("DB_USERNAME must be set in environment variables");
    let database_password =
        env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in environment variables");

    // connection database
    let mut cfg = tokio_postgres::Config::new();
    cfg.dbname(database_name.as_str());
    cfg.user(database_user.as_str());
    cfg.password(database_password.as_str());
    cfg.host(database_host.as_str());
    cfg.port(database_port.parse::<u16>().expect("Invalid port"));

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(cfg, NoTls, mgr_config);
    let pool = Arc::new(Pool::builder(mgr).max_size(16).build().unwrap());

    let sf = match Sonyflake::new() {
        Ok(sf) => sf, // หากได้ค่า Result::Ok(Sonyflake)
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to initialize Sonyflake: {}", e),
            ))
        } // หากมีข้อผิดพลาด
    };

    let snowflake_node = SnowflakeImpl::new(sf);

    // Master Data
    let master_data_repository = MasterDataImpl::new(Arc::clone(&pool));
    let master_data_use_case = MasterDataUseCaseImpl::new(master_data_repository);
    let master_data_handler = MasterDataHandler::new(master_data_use_case);
    let master_data_handler_data = web::Data::new(master_data_handler);

    // Task
    let task_repository = TaskImpl::new(Arc::clone(&pool), snowflake_node);
    let task_use_case = TaskUseCaseImpl::new(task_repository);
    let task_handler = TaskHandler::new(task_use_case);
    let task_handler_data = web::Data::new(task_handler);

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1")
                // Master Data
                .app_data(master_data_handler_data.clone())
                .configure(|cfg| {
                    configure_master_data_routes::<MasterDataUseCaseImpl<MasterDataImpl>>(cfg)
                })
                // Task management
                .app_data(task_handler_data.clone()) // Clone web::Data object
                .configure(|cfg| {
                    configure_task_routes::<TaskUseCaseImpl<TaskImpl<SnowflakeImpl>>>(cfg)
                }),
        )
    })
    .bind(("127.0.0.1", port))? // Specify Host and Port
    .run()
    .await
}
