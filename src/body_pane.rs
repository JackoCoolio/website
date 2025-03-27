use maud::{html, Markup, PreEscaped, Render};

use crate::events::{load_events, Event};

pub fn content() -> Markup {
    let events = load_events();

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
            date,
            content,
        } = self;

        html! {
            tr.event {
                td.timeline {
                    span { (date) }
                }
                td.body {
                    h1 { (title) }
                    p { (PreEscaped(content)) }
                }
            }
        }
    }
}
