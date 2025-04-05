use std::{io::Write, path::Path};

use axum::Router;
use comrak::{nodes::NodeValue, Arena};
use maud::{html, Markup, Render};
use serde::Deserialize;

use crate::summary_pane::{self, Navigation};

pub mod events;
pub mod projects;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Page {
    #[default] // default to events page
    Events,
    Projects,
}

impl Page {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Events => "Home",
            Self::Projects => "Projects",
        }
    }

    pub fn href(&self) -> &'static str {
        match self {
            Self::Events => "/",
            Self::Projects => "/projects",
        }
    }
}

impl Render for Page {
    fn render(&self) -> Markup {
        let (metadata, content) = match self {
            Page::Events => (PageMetadata::default(), events::content()),
            Page::Projects => (PageMetadata::default(), projects::content()),
        };

        let nav = Navigation {
            pages: [Page::Events, Page::Projects]
                .into_iter()
                .map(|page| (page, page == *self))
                .collect(),
        };

        page(
            metadata,
            html! {
                (summary_pane::container(nav))
                (content)
            },
        )
    }
}

pub fn router() -> Router {
    use axum::routing::get;
    Router::new()
        .route("/", get(|| async { Page::default().render() }))
        .route("/projects", get(|| async { Page::Projects.render() }))
}

#[derive(Default)]
struct PageMetadata {
    /// The subtitle of the page.
    subtitle: Option<String>,
}

fn page(meta: PageMetadata, inner: Markup) -> Markup {
    let subtitle = meta
        .subtitle
        .map(|st| format!(" - {st}"))
        .unwrap_or("".into());

    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                title { "Jackson Wambolt" (subtitle) }
                meta charset="utf-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                meta name="description" content="Jackson Wambolt's personal website";
                meta name="theme-color" content="#ffffff"; // TODO: set a real color
                link rel="stylesheet" href="style.css";
                script src="https://kit.fontawesome.com/c879508e2e.js" crossorigin="anonymous" {}
            }
            body {
                (inner)
            }
        }
    }
}

enum FromDocumentError {
    ReadFailure,
    WriteFailure,
    InvalidFrontMatter,
}

const FRONT_MATTER_DELIMITER: &str = "---";

trait FromDocument: Sized
where
    Self: 'static,
{
    type FrontMatter<'de>: Deserialize<'de>
    where
        Self: 'de;

    fn from_parts<'c, 'de>(front_matter: Self::FrontMatter<'de>, content: &'c str) -> Self
    where
        Self: 'static;

    fn from_document(path: impl AsRef<Path>) -> Result<Self, FromDocumentError> {
        let document =
            std::fs::read_to_string(path.as_ref()).map_err(|_| FromDocumentError::ReadFailure)?;

        let arena = Arena::new();

        let options = comrak::Options {
            extension: comrak::ExtensionOptions::builder()
                .front_matter_delimiter(FRONT_MATTER_DELIMITER.to_owned())
                .build(),
            ..Default::default()
        };

        let root = comrak::parse_document(&arena, &document, &options);

        let front_matter_delim: Vec<_> = FRONT_MATTER_DELIMITER.chars().collect();

        // get raw front-matter blob
        let front_matter = root
            .descendants()
            .find_map(|node| match &node.data.borrow().value {
                comrak::nodes::NodeValue::FrontMatter(x) => Some(x.clone()),
                _ => None,
            })
            .ok_or(FromDocumentError::InvalidFrontMatter)?;
        // trim '---' and newlines
        let front_matter = front_matter
            .trim()
            .trim_matches(front_matter_delim.as_slice())
            .trim();
        // parse
        let front_matter = serde_yaml::from_str(front_matter)
            .map_err(|_| FromDocumentError::InvalidFrontMatter)?;

        let content = {
            let mut buf = Vec::new();
            CustomFormatter::format_document(root, &options, &mut buf)
                .map_err(|_| FromDocumentError::WriteFailure)?;
            String::from_utf8_lossy(&buf).to_string()
        };

        Ok(Self::from_parts(front_matter, &content))
    }
}

comrak::create_formatter!(CustomFormatter, {
    NodeValue::Strong => |context, entering| {
        if entering {
            context.write_all(b"<span class=\"highlighted\"><span>")?;
        } else {
            context.write_all(b"</span><span class='highlight'></span><span class='shadow'></span></span>")?;
        }
    },
});

fn load_items_in_dir<D: FromDocument>(
    dir_path: impl AsRef<Path>,
) -> std::io::Result<impl Iterator<Item = D>> {
    Ok(std::fs::read_dir(dir_path)?.flatten().filter_map(|entry| {
        let file_type = entry.file_type().ok()?;
        if !file_type.is_file() {
            return None;
        }

        let file_name = entry.file_name(); // need separate let binding because returned OsStr is owned
        let file_name = file_name.to_str()?;
        if !file_name.ends_with(".md") {
            return None;
        }

        D::from_document(entry.path()).ok()
    }))
}
