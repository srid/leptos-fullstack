mod app;
mod thing;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    server::main()
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    app::main()
}
