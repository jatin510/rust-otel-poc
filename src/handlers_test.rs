#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, web, App};
    use std::sync::Mutex;

    use crate::handlers::{create_user, get_user, get_users, health_check, AppState};
    use crate::models::User;

    #[actix_web::test]
    async fn test_health_check() {
        let app =
            test::init_service(App::new().route("/health", web::get().to(health_check))).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        assert!(body_str.contains("healthy"));
    }

    #[actix_web::test]
    async fn test_get_users_empty() {
        let app_state = web::Data::new(AppState {
            users: Mutex::new(vec![]),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users", web::get().to(get_users)),
        )
        .await;

        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body_str, "[]");
    }

    #[actix_web::test]
    async fn test_create_and_get_user() {
        let app_state = web::Data::new(AppState {
            users: Mutex::new(vec![]),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users", web::post().to(create_user))
                .route("/users/{id}", web::get().to(get_user)),
        )
        .await;

        // Create a test user
        let user = User {
            id: "test_id".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        // Send POST request to create user
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&user)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::CREATED);

        // Now fetch the user
        let req = test::TestRequest::get().uri("/users/test_id").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        // Parse the response
        let body = test::read_body(resp).await;
        let response_user: User = serde_json::from_slice(&body).unwrap();

        assert_eq!(response_user.id, "test_id");
        assert_eq!(response_user.name, "Test User");
        assert_eq!(response_user.email, "test@example.com");
    }
}
