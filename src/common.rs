use maud::{html, Markup};

pub fn highlighted(text: &str) -> Markup {
    html! {
        span.highlighted {
            span { (text) }
            span.highlight {}
            span.shadow {}
        }
    }
}
