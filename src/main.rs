mod app;
#[cfg(feature = "ssr")]
mod server;
mod thing;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    server::main().await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    app::main()
}
