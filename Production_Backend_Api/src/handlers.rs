use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::Utc;
use validator::Validate;

use crate::{
    db::AppState,
    models::{CreateItemRequest, ErrorResponse, Item, ItemResponse},
};

pub async fn get_items(
    State(state): State<AppState>,
) -> Result<Json<Vec<ItemResponse>>, impl IntoResponse> {
    let items = match sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name, quantity, created_at
        FROM items
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch items: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal_server_error".to_string(),
                    details: std::collections::HashMap::from([(
                        "server".to_string(),
                        vec!["Failed to fetch items".to_string()],
                    )]),
                }),
            ));
        }
    };

    let response = items.into_iter().map(ItemResponse::from).collect();
    Ok(Json(response))
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Validate the request
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::from(validation_errors)),
        ));
    }

    // Insert into database
    match sqlx::query(
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
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            tracing::error!("Failed to create item: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal_server_error".to_string(),
                    details: std::collections::HashMap::from([(
                        "server".to_string(),
                        vec!["Failed to create item".to_string()],
                    )]),
                }),
            ))
        }
    }
}