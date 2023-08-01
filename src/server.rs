use crate::app::App;
use crate::thing::{ReadThings, Thing};
use axum::{
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::services::ServeDir;

pub async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;
    let client_dist = ServeDir::new(format!("{}/pkg", leptos_options.site_root.clone()));
    let app = Router::new()
        .route("/hello", get(root))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        .nest_service("/pkg", client_dist)
        .with_state(leptos_options);
    println!("Launching http://{}", &addr);
    println!("fn_url: {}", ReadThings::url());
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
