use maud::{html, Markup, PreEscaped, Render};

use crate::{common::highlighted, pages::load_items_in_dir};

use super::FromDocument;

pub fn content() -> Markup {
    let mut events: Vec<Event> = load_items_in_dir("public/events")
        .into_iter()
        .flatten()
        .collect();
    events.sort_by(|a, b| a.sort_key.cmp(&b.sort_key).reverse());

    html! {
        div #events {
            table {
                tbody {
                    @for event in events {
                        (event)
                    }
                }
            }
        }
    }
}

impl Render for Event {
    fn render(&self) -> Markup {
        let Event {
            title,
            start,
            end,
            content,
            ..
        } = self;

        html! {
            tr.event-header {
                td {
                    p { (start) }
                    @if let Some(end) = end {
                        p {
                            span { "to " }
                            @match end.as_str() {
                                "present" => (highlighted("present"))
                                _ => (end)
                            }
                        }
                    }
                }
                td {
                    h1 { (title) }
                }
            }
            tr.event-body {
                td {}
                td {
                    p { (PreEscaped(content)) }
                }
            }
        }
    }
}

struct Event {
    pub title: String,
    pub start: String,
    pub end: Option<String>,
    pub content: String,
    sort_key: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventFrontMatter<'a> {
    title: &'a str,
    start: &'a str,
    end: Option<&'a str>,
    sort_key: Option<&'a str>,
}

impl FromDocument for Event {
    type FrontMatter<'de> = EventFrontMatter<'de>;
    fn from_parts<'c, 'de>(front_matter: Self::FrontMatter<'de>, content: &'c str) -> Self
    where
        Self: 'static,
    {
        Self {
            title: front_matter.title.to_owned(),
            start: front_matter.start.to_owned(),
            end: front_matter.end.map(ToOwned::to_owned),
            sort_key: front_matter
                .sort_key
                .unwrap_or(front_matter.start)
                .to_owned(),
            content: content.to_owned(),
        }
    }
}
