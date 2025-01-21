use actix_web::{middleware::{ErrorHandlers, Logger}, web, App, HttpServer, Result};
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
        config::server::{load_env, ServerConfig},
        handlers::{
            master_data::{configure_routes as configure_master_data_routes, MasterDataHandler},
            task::{configure_routes as configure_task_routes, TaskHandler},
            user::{configure_routes as configure_user_routes, UserHandler},
            health_check::{configure_routes as config_health_check_routes, HealthCheckHandler},
        },
        repository::{health_check::HealthCheckRepositoriesImpl, master_data::MasterDataRepositoriesImpl, task::TaskRepositoriesImpl, user::UserRepositoriesImpl},
        use_case::{health_check::HealthCheckUseCaseImpl, master_data::MasterDataUseCaseImpl, task::TaskUseCaseImpl, user::UserUseCaseImpl},
    },
};

const ENV_FILE: &str = ".env.local";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env(ENV_FILE).expect("Failed to load environment variables.");
    let config = ServerConfig::from_env()?;

    let pool = create_db_pool(&config)?;
    let sonyflake = initialize_sonyflake()?;
    let snowflake_node = SnowflakeImpl::new(sonyflake);

    let health_check_handler_data = create_health_check_handler_data(Arc::clone(&pool));
    let master_data_handler_data = create_master_data_handler_data(Arc::clone(&pool));
    let task_handler_data = create_task_handler_data(Arc::clone(&pool), snowflake_node);
    let user_handler_data = create_user_handler_data(Arc::clone(&pool), &config);

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

                    // Health Check
                    .app_data(health_check_handler_data.clone())
                    .configure(|cfg| {
                        config_health_check_routes::<HealthCheckUseCaseImpl<HealthCheckRepositoriesImpl>>(cfg)
                    })
                    
                    // User
                    .app_data(user_handler_data.clone())
                    .configure(|cfg| {
                        configure_user_routes::<UserUseCaseImpl<UserRepositoriesImpl>>(cfg)
                    })

                    // Master Data
                    .app_data(master_data_handler_data.clone())
                    .configure(|cfg| {
                        configure_master_data_routes::<MasterDataUseCaseImpl<MasterDataRepositoriesImpl>>(cfg)
                    })

                    // Task Management
                    .app_data(task_handler_data.clone())
                    .configure(|cfg| {
                        configure_task_routes::<TaskUseCaseImpl<TaskRepositoriesImpl<SnowflakeImpl>>>(cfg, config.jwt_secret.clone())
                    })
                ,
            )
    })
    .bind(("0.0.0.0", config.api_port))
        .expect("Cannot bind to port 4000")
    .run()
    .await
}

fn create_db_pool(config: &ServerConfig) -> Result<Arc<Pool>, std::io::Error> {
    let mut db_cfg = tokio_postgres::Config::new();
    db_cfg
        .dbname(&config.database_name)
        .user(&config.database_user)
        .password(&config.database_password)
        .host(&config.database_host)
        .port(config.database_port);

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

fn create_health_check_handler_data(
    pool: Arc<Pool>,
) -> web::Data<HealthCheckHandler<HealthCheckUseCaseImpl<HealthCheckRepositoriesImpl>>> {
    let health_check_repository = HealthCheckRepositoriesImpl::new(pool);
    let health_check_use_case = HealthCheckUseCaseImpl::new(health_check_repository);
    let health_check_handler = HealthCheckHandler::new(health_check_use_case);
    web::Data::new(health_check_handler)
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
    config: &ServerConfig,
) -> web::Data<UserHandler<UserUseCaseImpl<UserRepositoriesImpl>>> {
    let user_repository = UserRepositoriesImpl::new(pool);
    let user_use_case = UserUseCaseImpl::new(user_repository, config.jwt_secret.clone());
    let user_handler = UserHandler::new(user_use_case);
    web::Data::new(user_handler)
}
