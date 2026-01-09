use axum::{routing::get, Router, serve};
use tokio::net::TcpListener;
use std::net::SocketAddr;

mod db;
mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", routes::create_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}