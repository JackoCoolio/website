use maud::{html, Markup, Render};

use crate::common::highlighted;

/// The container for the summary content.
///
/// Takes up the entire space so that the inner content can be aligned correctly.
pub fn container() -> Markup {
    html! {
        div #summary {
            (content())
        }
    }
}

struct SummaryField(&'static str, &'static str);

const FIELDS: &[SummaryField] = &[
    SummaryField("is a", "Software Developer"),
    SummaryField("at", "Epic"),
    SummaryField("in", "Madison, WI"),
];

impl maud::Render for SummaryField {
    fn render(&self) -> Markup {
        let SummaryField(label, value) = self;
        html! {
            li {
                (label)
                " "
                (value)
                br;
            }
        }
    }
}

fn content() -> Markup {
    html! {
        (portrait())
        div.text {
            h1.name { (highlighted("Jackson Wambolt")) }
            ul.fields {
                @for field in FIELDS {
                    (field)
                }
            }
        }
        (links())
    }
}

fn portrait() -> Markup {
    html! {
        div #portrait {
            img src="portrait-transparent.png";
        }
    }
}

struct Link(&'static str, &'static str, &'static str);

impl Render for Link {
    fn render(&self) -> Markup {
        let Link(fa_class, alt_title, href) = self;
        let class = format!("fa-{fa_class}");

        html! {
            a href=(href) {
                i.fa-brands.(class) title=(alt_title) {}
            }
        }
    }
}

const LINKS: &[Link] = &[
    Link("github", "GitHub", "https://github.com/JackoCoolio"),
    Link(
        "linkedin",
        "LinkedIn",
        "https://www.linkedin.com/in/jackson-wambolt/",
    ),
];

fn links() -> Markup {
    html! {
        div #links {
            @for link in LINKS {
                (link)
            }
        }
    }
}
