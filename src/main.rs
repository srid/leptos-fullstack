#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    leptos_fullstack::server::main().await
}
