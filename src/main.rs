use std::net::{IpAddr, SocketAddr};

use axum::{routing::get, Router};
use clap::{arg, value_parser, Command};
use maud::{html, Markup};
use tower_http::services::ServeDir;

mod body_pane;
mod common;
mod events;
mod summary_pane;

#[tokio::main]
async fn main() {
    let matches = Command::new("website")
        .arg(arg!(--host <HOST> "The host to bind to").value_parser(value_parser!(IpAddr)))
        .arg(arg!(--port <PORT> "The port").value_parser(value_parser!(u16)))
        .get_matches();

    let Some(host) = matches.get_one("host") else {
        eprintln!("specify a host with `--host`");
        std::process::exit(1);
    };

    let Some(port) = matches.get_one("port") else {
        eprintln!("specify a port with `--port`");
        std::process::exit(1);
    };

    let addr = SocketAddr::new(*host, *port);

    let app = Router::new()
        .route("/", get(index))
        .fallback_service(ServeDir::new("public"));

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
