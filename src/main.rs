use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use axum::{routing::get, Router};
use clap::Parser;
use maud::{html, Markup};
use tower_http::services::ServeDir;

mod body_pane;
mod common;
mod events;
mod summary_pane;

#[derive(clap::Parser)]
struct Args {
    #[arg(long)]
    host: IpAddr,
    #[arg(long, short = 'p')]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let app = Router::new()
        .route("/", get(index))
        .fallback_service(ServeDir::new("public"));

    let addr = SocketAddr::from((args.host, args.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[derive(Default)]
struct PageMetadata {
    /// The subtitle of the page.
    subtitle: Option<String>,
}

fn page(meta: PageMetadata, inner: Markup) -> Markup {
    let subtitle = meta
        .subtitle
        .map(|st| format!(" - {st}"))
        .unwrap_or("".into());

    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                title { "Jackson Wambolt" (subtitle) }
                meta charset="utf-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                meta name="description" content="Jackson Wambolt's personal website";
                meta name="theme-color" content="#ffffff"; // TODO: set a real color
                link rel="stylesheet" href="style.css";
                script src="https://kit.fontawesome.com/c879508e2e.js" crossorigin="anonymous" {}
            }
            body {
                (inner)
            }
        }
    }
}

async fn index() -> Markup {
    page(
        PageMetadata::default(),
        html! {
            (summary_pane::container())
            (body_pane::content())
        },
    )
}
