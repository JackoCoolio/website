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
            tr.event-header {
                td {
                    span { (date) }
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
