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
            img #portrait src="portrait.png";
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
