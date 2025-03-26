use maud::{html, Markup, Render};

const EVENTS: &[Event] = &[
    Event {
        title: "example title",
        date: "1/1/1970",
        content: "lorem ipsum dolor sit amet",
    },
    Event {
        title: "example title",
        date: "1/1/1970",
        content: "lorem ipsum dolor sit amet",
    },
];

pub fn content() -> Markup {
    html! {
        div #events {
            table {
                tbody {
                    @for event in EVENTS {
                        (event)
                    }
                }
            }
        }
    }
}

struct Event<'a> {
    title: &'a str,
    date: &'a str,
    content: &'a str,
}

impl Render for Event<'_> {
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
                    p { (content) }
                }
            }
        }
    }
}
