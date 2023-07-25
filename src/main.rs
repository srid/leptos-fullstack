use std::net::SocketAddr;

use axum::{routing::get, Router};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Sridhar Ratnakumar", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Launching http://localhost:3000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
