#[cfg(test)]
mod tests {
    use crate::internal::pkg::exceptions::error_message::FAIL_TO_LOAD_ENV;
    use crate::internal::pkg::middleware::jwt::create_token;
    use crate::internal::pkg::middleware::{error::add_error_header, response::ApiResponse};
    use crate::internal::server::config::server::{load_env, ServerConfig};
    use crate::internal::server::{
        domain::{
            entities::task::{
                Task,
                TaskID,
            },
            repositories::task::MockTaskRepositories,
        },
        handlers::task::{configure_routes, TaskHandler},
        request::task::TaskRequest,
        use_case::task::TaskUseCaseImpl,
    };
    use actix_web::middleware::ErrorHandlers;
    use actix_web::{http::header::ContentType, test, web, App};

    #[actix_web::test]
    async fn test_success_get_task() {
        let mock_data = vec![
            Task {
                id: 548753961092383042,
                title: "member".to_string(),
                description: None,
                task_status_id: Some(7250066663482068992),
                priority_levels_id: Some(7250065969870016512),
                created_by: CREATED_BY,
                created_at: Default::default(),
                updated_at: None,
                updated_by: None,
            },
            Task {
                id: 548753961092383042,
                title: "member2".to_string(),
                description: None,
                task_status_id: Some(7250066663482068992),
                priority_levels_id: Some(7250065969870016512),
                created_by: CREATED_BY,
                created_at: Default::default(),
                updated_at: None,
                updated_by: None,
            },
        ];

        load_env(".env.local").expect(FAIL_TO_LOAD_ENV);
        let config = ServerConfig::from_env().unwrap();

        const CREATED_BY: i64 = 1844995683120058368;
        let token = create_token(CREATED_BY, config.jwt_secret.clone().as_str());

        let mut mock_repo = MockTaskRepositories::new();
        mock_repo
            .expect_list_task()
            .returning(|| Ok(vec![
                Task {
                    id: 548753961092383042,
                    title: "member".to_string(),
                    description: None,
                    task_status_id: Some(7250066663482068992),
                    priority_levels_id: Some(7250065969870016512),
                    created_by: CREATED_BY,
                    created_at: Default::default(),
                    updated_at: None,
                    updated_by: None,
                },
                Task {
                    id: 548753961092383042,
                    title: "member2".to_string(),
                    description: None,
                    task_status_id: Some(7250066663482068992),
                    priority_levels_id: Some(7250065969870016512),
                    created_by: CREATED_BY,
                    created_at: Default::default(),
                    updated_at: None,
                    updated_by: None,
                },
            ]));

        let use_case = TaskUseCaseImpl::new(mock_repo);
        let handler = TaskHandler::new(use_case);
        let master_data_handler_data = web::Data::new(handler);

        let app = test::init_service(
            App::new()
                .wrap(ErrorHandlers::new().default_handler(add_error_header))
                .service(
                    web::scope("/api/v1")
                        .app_data(master_data_handler_data.clone())
                        .configure(|cfg| {
                            configure_routes::<TaskUseCaseImpl<MockTaskRepositories>>(cfg, config.jwt_secret.clone())
                        })
                    ,
                )
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/task")
            .insert_header(ContentType::json())
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = test::read_body(resp).await;
        let body: ApiResponse<Vec<Task>> = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body.status, "success");
        assert_eq!(body.message, "get task successfully");
        assert_eq!(body.data, mock_data);
    }

    #[actix_web::test]
    async fn test_success_create_task() {
        let task = TaskRequest {
            title: "member".to_string(),
            description: None,
            task_status_id: 7250066646188953600,
            priority_levels_id: 7250065969870016512,
        };


        load_env(".env.local").expect(FAIL_TO_LOAD_ENV);
        let config = ServerConfig::from_env().unwrap();

        const CREATED_BY: i64 = 1844995683120058368;
        const ID: i64 = 549543142173442373;
        let mock_data = TaskID {
            id: ID
        };
        let token = create_token(CREATED_BY, config.jwt_secret.clone().as_str());

        let mut mock_repo = MockTaskRepositories::new();
        mock_repo
            .expect_create_task()
            .returning(|_| Ok(ID));

        let use_case = TaskUseCaseImpl::new(mock_repo);
        let handler = TaskHandler::new(use_case);
        let master_data_handler_data = web::Data::new(handler);

        let app = test::init_service(
            App::new()
                .wrap(ErrorHandlers::new().default_handler(add_error_header))
                .service(
                    web::scope("/api/v1")
                        .app_data(master_data_handler_data.clone())
                        .configure(|cfg| {
                            configure_routes::<TaskUseCaseImpl<MockTaskRepositories>>(cfg, config.jwt_secret.clone())
                        })
                    ,
                )
        ).await;

        let req = test::TestRequest::post()
            .set_json(task)
            .uri("/api/v1/task")
            .insert_header(ContentType::json())
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body_bytes = test::read_body(resp).await;
        let body: ApiResponse<TaskID> = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body.status, "success");
        assert_eq!(body.message, "Task created successfully");
        assert_eq!(body.data, mock_data);
    }
}