use maud::{html, Markup};

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
        div {
            (portrait())
            div.text {
                h1.name { (highlighted("Jackson Wambolt")) }
                ul.fields {
                    @for field in FIELDS {
                        (field)
                    }
                }
            }
        }
    }
}

fn portrait() -> Markup {
    html! {
        div #portrait {
            img src="portrait-transparent.png";
        }
    }
}
