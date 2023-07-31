use axum::{
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use leptos::*;
use leptos_fullstack_common::Thing;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new(env!("CLIENT_DIST")))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/hello", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Launching http://localhost:3000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// The interface should match that of `common` crate
#[server(ReadThings, "/api", "Url", "read-things")]
pub async fn read_things() -> Result<Vec<Thing>, leptos::ServerFnError> {
    Ok(vec![
        Thing::new("Hello 1 from backend".to_string()),
        Thing::new("Hello 2 from backend".to_string()),
        Thing::new("Hello 3 from backend".to_string()),
    ])
}

#[debug_handler]
async fn root() -> String {
    let thing = Thing::new("Hello from backend".to_string());
    serde_json::to_string(&thing).unwrap()
}
