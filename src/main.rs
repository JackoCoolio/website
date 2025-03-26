use axum::{routing::get, Router};
use maud::{html, Markup};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

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
            h1 {
                "hello, world!"
            }
        },
    )
}
