use std::env;
use actix_web::{
    middleware::{ErrorHandlers, Logger},
    web, App, HttpServer, Result,
};

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use env_logger::Env;
use sonyflake::Sonyflake;
use std::sync::Arc;
use tokio_postgres::NoTls;

mod internal;
use internal::{
    pkg::{middleware::{
        error::add_error_header,
    }, utils::snowflake::SnowflakeImpl},
    server::{
        config::server::{load_env, parse_port_from_env, DatabaseConfig},
        handlers::{
            master_data::{configure_routes as configure_master_data_routes, MasterDataHandler},
            task::{configure_routes as configure_task_routes, TaskHandler},
            user::{configure_routes as configure_user_routes, UserHandler},
        },
        repository::{master_data::MasterDataRepositoriesImpl, task::TaskRepositoriesImpl, user::UserRepositoriesImpl},
        use_case::{master_data::MasterDataUseCaseImpl, task::TaskUseCaseImpl, user::UserUseCaseImpl},
    },
};
use crate::internal::pkg::middleware::auth::JwtMiddleware;

const ENV_FILE: &str = ".env.local";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env(ENV_FILE);
    let port = parse_port_from_env()?;
    let db_config = DatabaseConfig::from_env()?;
    let pool = create_db_pool(&db_config)?;
    let sonyflake = initialize_sonyflake()?;
    let snowflake_node = SnowflakeImpl::new(sonyflake);

    let master_data_handler_data = create_master_data_handler_data(Arc::clone(&pool));
    let task_handler_data = create_task_handler_data(Arc::clone(&pool), snowflake_node);
    let user_handler_data = create_user_handler_data(Arc::clone(&pool));

    let jwt_secret = env::var("JWT_SECRET").expect("DB_PASSWORD must be set in environment variables");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            // จัดการ logger
            .wrap(Logger::default())
            // จัดการ error response ทั้งหมดใน API
            .wrap(ErrorHandlers::new().default_handler(add_error_header))
            // รวม service
            .service(
                web::scope("/api/v1")



                    // User
                    .app_data(user_handler_data.clone())
                    .configure(|cfg| {
                        configure_user_routes::<UserUseCaseImpl<UserRepositoriesImpl>>(cfg)
                    })
                    .wrap(JwtMiddleware::new(jwt_secret.as_str())) // Add JWT
                    // Master Data
                    .app_data(master_data_handler_data.clone())
                    .configure(|cfg| {
                        configure_master_data_routes::<MasterDataUseCaseImpl<MasterDataRepositoriesImpl>>(cfg)
                    })

                    // Task Management
                    .app_data(task_handler_data.clone())
                    .configure(|cfg| {
                        configure_task_routes::<TaskUseCaseImpl<TaskRepositoriesImpl<SnowflakeImpl>>>(cfg)
                    })
                ,
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn create_db_pool(config: &DatabaseConfig) -> Result<Arc<Pool>, std::io::Error> {
    let mut db_cfg = tokio_postgres::Config::new();
    db_cfg
        .dbname(&config.name)
        .user(&config.user)
        .password(&config.password)
        .host(&config.host)
        .port(config.port);

    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let manager = Manager::from_config(db_cfg, NoTls, manager_config);
    Ok(Arc::new(
        Pool::builder(manager).max_size(16).build().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create database pool: {}", e),
            )
        })?,
    ))
}

fn initialize_sonyflake() -> Result<Sonyflake, std::io::Error> {
    let sonyflake = Sonyflake::new().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to initialize Sonyflake: {}", e),
        )
    })?;
    Ok(sonyflake)
}

fn create_master_data_handler_data(
    pool: Arc<Pool>,
) -> web::Data<MasterDataHandler<MasterDataUseCaseImpl<MasterDataRepositoriesImpl>>> {
    let master_data_repository = MasterDataRepositoriesImpl::new(pool);
    let master_data_use_case = MasterDataUseCaseImpl::new(master_data_repository);
    let master_data_handler = MasterDataHandler::new(master_data_use_case);
    web::Data::new(master_data_handler)
}

fn create_task_handler_data(
    pool: Arc<Pool>,
    snowflake_node: SnowflakeImpl,
) -> web::Data<TaskHandler<TaskUseCaseImpl<TaskRepositoriesImpl<SnowflakeImpl>>>> {
    let task_repository = TaskRepositoriesImpl::new(pool, snowflake_node);
    let task_use_case = TaskUseCaseImpl::new(task_repository);
    let task_handler = TaskHandler::new(task_use_case);
    web::Data::new(task_handler)
}

fn create_user_handler_data(
    pool: Arc<Pool>,
) -> web::Data<UserHandler<UserUseCaseImpl<UserRepositoriesImpl>>> {
    let user_repository = UserRepositoriesImpl::new(pool);
    let user_use_case = UserUseCaseImpl::new(user_repository);
    let user_handler = UserHandler::new(user_use_case);
    web::Data::new(user_handler)
}
