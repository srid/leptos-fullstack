use axum::{routing::get, Router};
use axum_macros::debug_handler;
use leptos_fullstack_common::Thing;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new(env!("CLIENT_DIST")))
        .route("/hello", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Launching http://localhost:3000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn root() -> String {
    let thing = Thing::new("Hello from backend".to_string());
    serde_json::to_string(&thing).unwrap()
}
