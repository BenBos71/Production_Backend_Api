// tests/items_api.rs
use axum::{
    body,
    http::{Method, StatusCode, header, Request},
    Router,
    routing::post,
};
use serde_json::json;
use sqlx::{SqlitePool, migrate::MigrateDatabase};
use tower::ServiceExt;

// Inline test setup
async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create test database pool");
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    pool
}

// Inline test app creation
async fn create_test_app() -> Router {
    let pool = setup_test_db().await;
    use production_backend_api::{db::AppState, create_router};

    let state = AppState::new(pool);
    create_router(state)
}

#[tokio::test]
async fn test_create_and_get_items() {
    // Set up test database and app
    let app = create_test_app().await;

    // Test data
    let test_item = json!({
        "name": "Test Item",
        "quantity": 42
    });

    // Test POST /api/items
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/items")
                .header(header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(serde_json::to_vec(&test_item).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Verify POST was successful
    assert_eq!(response.status(), StatusCode::CREATED);

    // Test GET /api/items
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/items")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Verify GET was successful
    assert_eq!(response.status(), StatusCode::OK);

    let body = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    // Verify the response contains exactly one item
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["name"], "Test Item");
    assert_eq!(items[0]["quantity"], 42);
}

#[tokio::test]
async fn test_create_item_validation() {
    // Set up test database and app
    let app = create_test_app().await;

    // Test data with invalid quantity (negative)
    let invalid_item = json!({
        "name": "Invalid Item",
        "quantity": -1
    });

    // Test POST /api/items with invalid data
    let (status, _) = make_request(
        app,
        axum::http::Method::POST,
        "/api/items",
        Some(invalid_item),
    )
    .await;

    // Verify validation failed with 400 Bad Request
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_item_empty_name() {
    // Set up test database and app
    let app = create_test_app().await;

    // Test data with empty name
    let invalid_item = json!({
        "name": "",  // Empty name should fail validation
        "quantity": 10
    });

    // Test POST /api/items with empty name
    let (status, _) = make_request(
        app,
        axum::http::Method::POST,
        "/api/items",
        Some(invalid_item),
    )
    .await;

    // Verify validation failed with 400 Bad Request
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
