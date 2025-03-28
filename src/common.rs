use maud::{html, Markup};

pub fn highlighted(text: &str) -> Markup {
    html! {
        span.highlighted {
            span { (text) }
            div.highlight {}
            div.shadow {}
        }
    }
}
