// tests/common.rs
use axum::{
    body::{self, Body},
    http::{Request, StatusCode},
};
use sqlx::SqlitePool;
use tower::ServiceExt;

use production_backend_api::{
    build_app,
    db::AppState,
    models::ItemResponse,
};

pub async fn setup_test_db() -> SqlitePool {
    // Create an in-memory SQLite database
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create test database pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub async fn create_test_app(pool: SqlitePool) -> axum::Router {
    let state = AppState::new(pool);
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);
    
    build_app(state, cors).await.expect("Failed to build app")
}

pub async fn make_request(
    app: axum::Router,
    method: axum::http::Method,
    uri: &str,
    body: Option<serde_json::Value>,
) -> (StatusCode, Option<Vec<ItemResponse>>) {
    let body = body.map(|b| Body::from(serde_json::to_vec(&b).unwrap()));
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("Content-Type", "application/json");

    let request = if let Some(body) = body {
        request.body(body).unwrap()
    } else {
        request.body(Body::empty()).unwrap()
    };

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();

    let body = body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    
    let items = if !body.is_empty() {
        if let Ok(items) = serde_json::from_slice::<Vec<ItemResponse>>(&body) {
            Some(items)
        } else {
            None
        }
    } else {
        None
    };

    (status, items)
}