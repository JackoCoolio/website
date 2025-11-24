use maud::{html, Markup, Render};

use crate::{common::highlighted, pages::Page};

/// The container for the summary content.
///
/// Takes up the entire space so that the inner content can be aligned correctly.
pub fn container(nav: Navigation) -> Markup {
    html! {
        div #summary {
            (content())
            (nav)
        }
    }
}

struct SummaryField(&'static str, &'static str);

const FIELDS: &[SummaryField] = &[
    SummaryField("is a", "Software Developer"),
    SummaryField("at", "Google"),
    SummaryField("in", "San Francisco, CA"),
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

struct Link {
    category: &'static str,
    icon: &'static str,
    alt_text: &'static str,
    href: &'static str,
}

impl Render for Link {
    fn render(&self) -> Markup {
        let Link {
            category,
            icon,
            alt_text,
            href,
        } = self;
        let category_class = format!("fa-{category}");
        let icon_class = format!("fa-{icon}");

        html! {
            a href=(href) {
                i.(category_class).(icon_class) title=(alt_text) {}
            }
        }
    }
}

const LINKS: &[Link] = &[
    Link {
        category: "brands",
        icon: "github",
        alt_text: "GitHub",
        href: "https://github.com/JackoCoolio",
    },
    Link {
        category: "brands",
        icon: "linkedin",
        alt_text: "LinkedIn",
        href: "https://www.linkedin.com/in/jackson-wambolt/",
    },
    Link {
        category: "solid",
        icon: "file",
        alt_text: "Resume",
        href: "/resume.pdf",
    },
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

pub struct Navigation {
    pub pages: Box<[(Page, bool)]>,
}

impl Render for Navigation {
    fn render(&self) -> Markup {
        html! {
            nav {
                ul {
                    @for (page, is_selected) in &self.pages {
                        li.selected[*is_selected] {
                            a href=(page.href()) {
                                @let text = page.display_name();
                                @if *is_selected {
                                    (highlighted(text))
                                } @else {
                                    (text)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
