use maud::{html, Markup, PreEscaped, Render};

use super::{load_items_in_dir, FromDocument};

pub fn content() -> Markup {
    let mut projects: Vec<Project> = load_items_in_dir("public/projects")
        .into_iter()
        .flatten()
        .collect();
    projects.sort_by(|a, b| a.sort_key.cmp(&b.sort_key));

    html! {
        div #projects {
            @for prj in projects {
                (prj)
            }
        }
    }
}

struct Project {
    title: String,
    content: String,
    sort_key: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FrontMatter<'a> {
    title: &'a str,
    sort_key: &'a str,
}

impl FromDocument for Project {
    type FrontMatter<'de> = FrontMatter<'de>;

    fn from_parts<'c, 'de>(front_matter: Self::FrontMatter<'de>, content: &'c str) -> Self
    where
        Self: 'static,
    {
        Self {
            title: front_matter.title.to_owned(),
            sort_key: front_matter.sort_key.to_owned(),
            content: content.to_owned(),
        }
    }
}

impl Render for Project {
    fn render(&self) -> Markup {
        let Project { title, content, .. } = self;

        html! {
            div.project {
                h1 { (title) }
                p { (PreEscaped(content)) }
            }
        }
    }
}
