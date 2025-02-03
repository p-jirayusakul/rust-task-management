#[cfg(test)]
mod tests {
    use actix_web::middleware::ErrorHandlers;
    use actix_web::{http::header::ContentType, test, web, App};
    use crate::application::use_cases::master_data::MasterDataUseCaseImpl;
    use crate::domain::entities::master_data::MasterDataTaskStatus;
    use crate::domain::repositories::master_data::MockMasterDataRepositories;
    use crate::infrastructure::api::handlers::master_data_handler::MasterDataHandler;
    use crate::infrastructure::api::routes::master_data_routes::configure_master_data_routes;
    use crate::shared::exceptions::custom_error::CustomError;
    use crate::shared::middleware::errors::add_error_header;
    use crate::shared::middleware::response::{ApiResponse, ApiResponseErr};

    #[actix_web::test]
    async fn test_success_master_data_task_status() {
        let mock_data = vec![
            MasterDataTaskStatus {
                id: 7250066646188953600,
                title: "Pending".to_string(),
                code: "PENDING".to_string(),
            },
            MasterDataTaskStatus {
                id: 7250066663482068992,
                title: "In Progress".to_string(),
                code: "IN_PROGRESS".to_string(),
            },
            MasterDataTaskStatus {
                id: 7250066683811860480,
                title: "Completed".to_string(),
                code: "COMPLETED".to_string(),
            },
        ];

        let mut mock_repo = MockMasterDataRepositories::new();
        mock_repo
            .expect_list_task_status()
            .returning(|| Ok(vec![
                MasterDataTaskStatus {
                    id: 7250066646188953600,
                    title: "Pending".to_string(),
                    code: "PENDING".to_string(),
                },
                MasterDataTaskStatus {
                    id: 7250066663482068992,
                    title: "In Progress".to_string(),
                    code: "IN_PROGRESS".to_string(),
                },
                MasterDataTaskStatus {
                    id: 7250066683811860480,
                    title: "Completed".to_string(),
                    code: "COMPLETED".to_string(),
                },
            ]));

        let use_case = MasterDataUseCaseImpl::new(mock_repo);
        let handler = MasterDataHandler::new(use_case);
        let master_data_handler_data = web::Data::new(handler);

        let app = test::init_service(
            App::new()
                .wrap(ErrorHandlers::new().default_handler(add_error_header))
                .service(
                    web::scope("/api/v1")
                        .app_data(master_data_handler_data.clone())
                        .configure(|cfg| {
                            configure_master_data_routes::<MasterDataUseCaseImpl<MockMasterDataRepositories>>(cfg)
                        })
                    ,
                )
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/master-data/task-status")
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = test::read_body(resp).await;
        let body: ApiResponse<Vec<MasterDataTaskStatus>> = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body.status, "success");
        assert_eq!(body.message, "get list task status successfully");
        assert_eq!(body.data, mock_data);
    }

    #[actix_web::test]
    async fn test_inter_server_error_master_data_task_status() {
        let mut mock_repo = MockMasterDataRepositories::new();
        mock_repo
            .expect_list_task_status()
            .returning(|| Err(CustomError::RepositoryError("Database query failed".to_string())));

        let use_case = MasterDataUseCaseImpl::new(mock_repo);
        let handler = MasterDataHandler::new(use_case);
        let master_data_handler_data = web::Data::new(handler);

        let app = test::init_service(
            App::new()
                .wrap(ErrorHandlers::new().default_handler(add_error_header))
                .service(
                    web::scope("/api/v1")
                        .app_data(master_data_handler_data.clone())
                        .configure(|cfg| {
                            configure_master_data_routes::<MasterDataUseCaseImpl<MockMasterDataRepositories>>(cfg)
                        })
                    ,
                )
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/master-data/task-status")
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_server_error());

        let body_bytes = test::read_body(resp).await;
        let body: ApiResponseErr = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body.status, "error");
        assert_eq!(body.message, "Internal Server Error");
    }
}