use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;
use std::env;

mod internal;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if dotenv::from_filename(".env.local").is_err() {
        println!("Warning: .env.local not found. Using OS environment variables instead.");
    }

    let port_string = env::var("APP_PORT").expect("APP_PORT must be set in environment variables");
    let port: u16 = port_string.parse().expect("Failed to parse port as u16");
    let database_host = env::var("DB_HOST").expect("DB_HOST must be set in environment variables");
    let database_port = env::var("DB_PORT").expect("DB_PORT must be set in environment variables");
    let database_name = env::var("DB_DATABASE").expect("DB_DATABASE must be set in environment variables");
    let database_user = env::var("DB_USERNAME").expect("DB_USERNAME must be set in environment variables");
    let database_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in environment variables");

    let (client, connection) = match tokio_postgres::connect(
        &format!(
            "host={} port={} dbname={} user={} password={}",
            database_host,
            database_port,
            database_name,
            database_user,
            database_password,
        ),
        NoTls,
    )
        .await
    {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database connection error",
            ));
        }
    };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // สร้าง repository instance
    let master_data_repository = internal::server::repository::master_data::MasterDataImpl::new(client);

    // สร้าง instance ของ UseCase สำหรับ MasterData
    let master_data_use_case = internal::server::use_case::master_data::MasterDataUseCaseImpl::new(master_data_repository);

    // สร้าง instance ของ MasterDataHandler
    let master_data_handler = internal::server::handlers::master_data::MasterDataHandler::new(
        master_data_use_case,
    );

    // ใช้ web::Data เพื่อทำให้ง่ายในการ clone
    let master_data_handler_data = web::Data::new(master_data_handler);

    // สร้าง HttpServer พร้อมระบุ routes
    HttpServer::new(move || {
        App::new()
            .app_data(master_data_handler_data.clone()) // Clone web::Data object
            .service(
                web::scope("/api/v1")
                    // Master Data
                    .configure(internal::server::handlers::master_data::configure_routes::<
                        internal::server::use_case::master_data::MasterDataUseCaseImpl<
                            internal::server::repository::master_data::MasterDataImpl,
                        >,
                    >),
            )
    })
        .bind(("127.0.0.1", port))? // ระบุ Host และ Port
        .run()
        .await
}