use std::net::{IpAddr, SocketAddr};

use clap::Parser;
use tower_http::services::ServeDir;

mod common;
mod pages;
mod summary_pane;

#[derive(clap::Parser)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: IpAddr,
    #[arg(long, short = 'p')]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let app = pages::router().fallback_service(ServeDir::new("public"));

    let addr = SocketAddr::from((args.host, args.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
