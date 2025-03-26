use maud::{html, Markup};

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

fn content() -> Markup {
    html! {
        div {
            (portrait())
            div.text {
                h1.name { "Jackson Wambolt" }
                p {
                    "Software Developer" br;
                    "at Epic" br;
                    "in Madison, WI"
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
