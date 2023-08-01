use crate::app::App;
use crate::thing::{ReadThings, Thing};
use axum::http::StatusCode;
use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    extract::State,
    http::{Request, Uri},
    response::IntoResponse,
};
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
    let client_dist = ServeDir::new(leptos_options.site_root.clone());
    let app = Router::new()
        // custom routes
        .route("/hello", get(root))
        // server functions API routes
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        // application routes
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        // when none of the routes match the requested URL
        .fallback(error_handler)
        // static files are served as fallback (but *before* falling back to
        // error handler)
        .fallback_service(client_dist.clone())
        .with_state(leptos_options);
    println!("Launching http://{}", &addr);
    println!("fn_url: {}", ReadThings::url());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn error_handler(
    _uri: Uri,
    State(options): State<LeptosOptions>,
    _req: Request<Body>,
) -> AxumResponse {
    // TODO: Let app render error page
    let _handler =
        leptos_axum::render_app_to_stream(options.to_owned(), move |cx| view! {cx, <App/>});
    (StatusCode::NOT_FOUND, "'tis not found").into_response()
}

#[debug_handler]
async fn root() -> String {
    let thing = Thing::new("Hello from backend".to_string());
    serde_json::to_string(&thing).unwrap()
}
