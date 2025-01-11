use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;

mod internal;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let (client, connection) = match tokio_postgres::connect("host=localhost user=postgres password=1234 dbname=task_management", NoTls).await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection error"));
        }
    };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });


    // สร้าง repository instance
    let repository = internal::server::repository::master_data::MasterDataImpl::new(Arc::new(client));

    // สร้าง instance ของ UseCase สำหรับ MasterData
    let use_case = internal::server::use_case::master_data::MasterDataUseCaseImpl::new(repository);

    // สร้าง instance ของ MasterDataHandler
    let master_data_handler = internal::server::handlers::master_data::MasterDataHandler::new(
        use_case,
    );

    // สร้าง HttpServer พร้อมระบุ routes
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api/v1")
                    // Master Data
                    .app_data(web::Data::new(master_data_handler.clone()))
                    .configure(internal::server::handlers::master_data::configure_routes::<internal::server::use_case::master_data::MasterDataUseCaseImpl<internal::server::repository::master_data::MasterDataImpl>>)
            )
    })
    .bind(("127.0.0.1", 4000))? // ระบุ Host และ Port
    .run()
    .await
}