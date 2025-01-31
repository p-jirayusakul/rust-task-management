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
pub use internal::{
    pkg::{
        exceptions::error_message::FAIL_TO_LOAD_ENV, // Custom error message
        middleware::error::add_error_header,         // Middleware สำหรับจัดการ error header
        utils::snowflake::SnowflakeImpl,             // สำหรับสร้าง unique ID
    },
    server::{
        config::server::{load_env, ServerConfig}, // การโหลด config ของ server
        handlers::{
            health_check::{configure_routes as config_health_check_routes, HealthCheckHandler}, // Config routes สำหรับ master data
            master_data::{configure_routes as configure_master_data_routes, MasterDataHandler}, // Config routes สำหรับ task
            task::{configure_routes as configure_task_routes, TaskHandler}, // Config routes สำหรับ user
            user::{configure_routes as configure_user_routes, UserHandler}, // Config routes สำหรับ health check
        },
        repository::{
            health_check::HealthCheckRepositoriesImpl, // Repository สำหรับ health check
            master_data::MasterDataRepositoriesImpl,   // Repository สำหรับ master data
            task::TaskRepositoriesImpl,                // Repository สำหรับ task
            user::UserRepositoriesImpl,                // Repository สำหรับ user
        },
        use_case::{
            health_check::HealthCheckUseCaseImpl, // Use case logic สำหรับ health check
            master_data::MasterDataUseCaseImpl,   // Use case logic สำหรับ master data
            task::TaskUseCaseImpl,                // Use case logic สำหรับ task
            user::UserUseCaseImpl,                // Use case logic สำหรับ user
        },
    },
};

// ตั้งค่าที่อยู่ไฟล์ environment
const ENV_FILE: &str = ".env.local";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // โหลด environment และ config
    load_env(ENV_FILE).expect(FAIL_TO_LOAD_ENV);
    let config = ServerConfig::from_env()?; // โหลด config จาก environment

    // ===== Stage 1: Setup Handler =====
    // สร้าง connection pool สำหรับ database
    let max_size: usize = 16;
    let pool = create_db_pool(&config, max_size)?;
    let shutdown_pool = Arc::clone(&pool); // Clone Database Pool เพื่อใช้ใน Cleanup

    // สร้าง Sonyflake instance สำหรับการ generate unique ID
    // ใช้ Sonyflake สำหรับสร้าง Snowflake node
    let sonyflake = initialize_sonyflake()?;
    let snowflake_node = SnowflakeImpl::new(sonyflake);

    // เตรียม data handler สำหรับแต่ละ endpoint
    let health_check_handler_data = create_health_check_handler_data(Arc::clone(&pool));
    let master_data_handler_data = create_master_data_handler_data(Arc::clone(&pool));
    let task_handler_data = create_task_handler_data(Arc::clone(&pool), snowflake_node);
    let user_handler_data = create_user_handler_data(Arc::clone(&pool), &config);

    // ตั้งค่า logging จาก environment
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // ===== Stage 2: Run Server =====
    let server =
        HttpServer::new(move || {
            App::new()
                // Middleware สำหรับ logging request ยกเว้น health-check
                .wrap(
                    Logger::default().exclude_regex(r"/health-check/"), // ลดการ logging ของ health-check
                )

                // Middleware สำหรับจัดการ error response
                .wrap(ErrorHandlers::new().default_handler(add_error_header))

                // กำหนด API
                .service(
                    web::scope("/api/v1")
                        // Health Check routes
                        .app_data(health_check_handler_data.clone())
                        .configure(|cfg| {
                            config_health_check_routes::<
                                HealthCheckUseCaseImpl<HealthCheckRepositoriesImpl>,
                            >(cfg)
                        })

                        // User routes
                        .app_data(user_handler_data.clone())
                        .configure(|cfg| {
                            configure_user_routes::<UserUseCaseImpl<UserRepositoriesImpl>>(cfg)
                        })

                        // Master Data routes
                        .app_data(master_data_handler_data.clone())
                        .configure(|cfg| {
                            configure_master_data_routes::<
                                MasterDataUseCaseImpl<MasterDataRepositoriesImpl>,
                            >(cfg)
                        })

                        // Task Management routes
                        .app_data(task_handler_data.clone())
                        .configure(|cfg| {
                            configure_task_routes::<
                                TaskUseCaseImpl<TaskRepositoriesImpl<SnowflakeImpl>>,
                            >(cfg, config.jwt_secret.clone())
                        }),
                )
        })
        .bind(("0.0.0.0", config.api_port))
        .expect(&format!("Cannot bind to port {}", config.api_port));

    let server = server.run();

    // ===== Stage 3: Wait signal CTRL+C =====
    tokio::select! {
        res = server => {
            if let Err(err) = res {
                eprintln!("Server error: {}", err);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!("Received shutdown signal. Closing resources...");
        }
    }

    // ===== Stage 4: Close All connection e.g. database, redis .. =====
    close_connection_db(shutdown_pool);

    println!("Shutdown completed.");
    Ok(())
}

// ฟังก์ชันสำหรับสร้าง Database Connection Pool
fn create_db_pool(config: &ServerConfig, max_size: usize) -> Result<Arc<Pool>, std::io::Error> {
    let mut db_cfg = tokio_postgres::Config::new();
    db_cfg
        .dbname(&config.database_name)
        .user(&config.database_user)
        .password(&config.database_password)
        .host(&config.database_host)
        .port(config.database_port);

    // ตั้งค่าการรีไซเคิล connection pool
    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let manager = Manager::from_config(db_cfg, NoTls, manager_config);
    Ok(Arc::new(
        Pool::builder(manager).max_size(max_size).build().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create database pool: {}", e),
            )
        })?,
    ))
}

fn close_connection_db(pool: Arc<Pool>) -> () {
    pool.close();
    if pool.is_closed() {
        println!("Database connection pool closed successfully.");
    }
}

// ฟังก์ชันสำหรับสร้าง Sonyflake Instance
fn initialize_sonyflake() -> Result<Sonyflake, std::io::Error> {
    let sonyflake = Sonyflake::new().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to initialize Sonyflake: {}", e),
        )
    })?;
    Ok(sonyflake)
}

// ฟังก์ชันสำหรับสร้าง Health Check Data Handler
fn create_health_check_handler_data(
    pool: Arc<Pool>,
) -> web::Data<HealthCheckHandler<HealthCheckUseCaseImpl<HealthCheckRepositoriesImpl>>> {
    let health_check_repository = HealthCheckRepositoriesImpl::new(pool);
    let health_check_use_case = HealthCheckUseCaseImpl::new(health_check_repository);
    let health_check_handler = HealthCheckHandler::new(health_check_use_case);
    web::Data::new(health_check_handler)
}

// ฟังก์ชันสำหรับสร้าง Master Data Handler
fn create_master_data_handler_data(
    pool: Arc<Pool>,
) -> web::Data<MasterDataHandler<MasterDataUseCaseImpl<MasterDataRepositoriesImpl>>> {
    let master_data_repository = MasterDataRepositoriesImpl::new(pool);
    let master_data_use_case = MasterDataUseCaseImpl::new(master_data_repository);
    let master_data_handler = MasterDataHandler::new(master_data_use_case);
    web::Data::new(master_data_handler)
}

// ฟังก์ชันสำหรับสร้าง Task Handler
fn create_task_handler_data(
    pool: Arc<Pool>,
    snowflake_node: SnowflakeImpl,
) -> web::Data<TaskHandler<TaskUseCaseImpl<TaskRepositoriesImpl<SnowflakeImpl>>>> {
    let task_repository = TaskRepositoriesImpl::new(pool, snowflake_node);
    let task_use_case = TaskUseCaseImpl::new(task_repository);
    let task_handler = TaskHandler::new(task_use_case);
    web::Data::new(task_handler)
}

// ฟังก์ชันสำหรับสร้าง User Handler
fn create_user_handler_data(
    pool: Arc<Pool>,
    config: &ServerConfig,
) -> web::Data<UserHandler<UserUseCaseImpl<UserRepositoriesImpl>>> {
    let user_repository = UserRepositoriesImpl::new(pool);
    let user_use_case = UserUseCaseImpl::new(user_repository, config.jwt_secret.clone()); // UseCase logic
    let user_handler = UserHandler::new(user_use_case);
    web::Data::new(user_handler)
}
