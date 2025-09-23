use axum::{
    body::{self, Body},
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use sqlx::Executor;

use api::{app_routes, init_db_pool}; // 👈 reuse helpers from lib.rs

async fn setup_test_db() -> sqlx::PgPool {
    let pool = init_db_pool().await;
    // clean state before running the test
    pool.execute("TRUNCATE users CASCADE").await.unwrap();
    pool
}

#[tokio::test]
async fn test_create_and_list_users() {
    let pool = setup_test_db().await;
    let app = app_routes(pool.clone());

    // create a user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"username":"bob","email":"bob@tidasone.com"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // list users
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = body::to_bytes(response.into_body(), 65_536).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert!(body_str.contains("bob@tidasone.com"));
}

