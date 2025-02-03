use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::{ErrorHandlers, Logger};
use env_logger::Env;

mod domain;
mod shared;
mod infrastructure;
mod application;

use crate::application::use_cases::{
    auth::AuthUseCaseImpl, health_check::HealthCheckUseCaseImpl,
    master_data::MasterDataUseCaseImpl, task::TaskUseCaseImpl,
};

use crate::infrastructure::{
    api::{
        factories::{
            auth::create_user_handler_data, health_check::create_health_check_handler_data,
            master_data::create_master_data_handler_data, task::create_task_handler_data,
        },
        routes::{
            auth::configure_user_routes, health_check::config_health_check_routes,
            master_data_routes::configure_master_data_routes, task::configure_task_routes,
        },
    },
    config::{load_env, ServerConfig},
    database::{
        auth::AuthRepositoriesImpl,
        connection::{close_connection_db, create_db_pool},
        health_check::HealthCheckRepositoriesImpl,
        master_data::MasterDataRepositoriesImpl,
        task::TaskRepositoriesImpl,
    },
};

use crate::shared::{
    exceptions::error_message::FAIL_TO_LOAD_ENV,
    middleware::errors::add_error_header,
    utils::snowflake::{initialize_sonyflake, SnowflakeImpl},
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
                            configure_user_routes::<AuthUseCaseImpl<AuthRepositoriesImpl>>(cfg)
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