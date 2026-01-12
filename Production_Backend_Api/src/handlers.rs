use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use chrono::Utc;

use crate::{db::AppState, models::*};

pub async fn get_items(
    State(state): State<AppState>,
) -> Result<Json<Vec<ItemResponse>>, StatusCode> {
    let items = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name, quantity, created_at
        FROM items
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(ItemResponse::from)
    .collect();

    Ok(Json(items))
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> Result<StatusCode, StatusCode> {
    let _ = sqlx::query(
        r#"
        INSERT INTO items (name, quantity, created_at)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(&payload.name)
    .bind(payload.quantity)
    .bind(Utc::now())
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}