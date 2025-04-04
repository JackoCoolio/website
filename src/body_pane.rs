use maud::{html, Markup, PreEscaped, Render};

use crate::{
    common::highlighted,
    events::{load_events, Event},
};

pub fn content() -> Markup {
    let mut events = load_events().unwrap_or([].into());
    events.reverse();

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
