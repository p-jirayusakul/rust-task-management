use std::sync::Arc;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use crate::infrastructure::config::ServerConfig;

pub fn create_db_pool(config: &ServerConfig, max_size: usize) -> Result<Arc<Pool>, std::io::Error> {
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

pub fn close_connection_db(pool: Arc<Pool>) -> () {
    pool.close();
    if pool.is_closed() {
        println!("Database connection pool closed successfully.");
    }
}