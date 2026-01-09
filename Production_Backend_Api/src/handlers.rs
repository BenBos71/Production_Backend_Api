use axum::response::Json;
use serde_json::json;

pub async fn list_items() -> Json<serde_json::Value> {
    Json(json!({"message": "list items"}))
}

pub async fn get_item() -> Json<serde_json::Value> {
    Json(json!({"message": "get item"}))
}