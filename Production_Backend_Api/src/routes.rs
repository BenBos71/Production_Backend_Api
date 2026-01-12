use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    db::AppState,
    handlers::{create_item, get_items},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/items", get(get_items).post(create_item))
        .with_state(state)
}