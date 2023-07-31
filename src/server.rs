use crate::thing::Thing;
use axum::{
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub async fn main() {
    let client_dist = ServeDir::new(env!("CLIENT_DIST"));
    println!("Serving static files from {}", env!("CLIENT_DIST"));
    let app = Router::new()
        .nest_service("/", client_dist)
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
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
