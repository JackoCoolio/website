use axum::Router;
use maud::{html, Markup, Render};

use crate::summary_pane;

pub mod events;

#[derive(Default)]
pub enum Page {
    #[default] // default to events page
    Events,
}

impl Render for Page {
    fn render(&self) -> Markup {
        let (metadata, content) = match self {
            Page::Events => (PageMetadata::default(), events::content()),
        };

        page(
            metadata,
            html! {
                (summary_pane::container())
                (content)
            },
        )
    }
}

pub fn router() -> Router {
    use axum::routing::get;
    Router::new().route("/", get(|| async { Page::default().render() }))
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
