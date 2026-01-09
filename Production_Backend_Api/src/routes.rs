use axum::Router;
use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .route("/items", axum::routing::get(handlers::list_items))
        .route("/items/:id", axum::routing::get(handlers::get_item))
}