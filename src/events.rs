use std::{io::Write, path::Path};

use comrak::{create_formatter, nodes::NodeValue, parse_document, Arena, ExtensionOptions};
use serde::Deserialize;

const EVENTS_DIR: &str = "public/events";
const FRONT_MATTER_DELIMITER: &str = "---";

pub struct Event {
    pub title: String,
    pub start: String,
    pub end: Option<String>,
    pub content: String,
    sort_key: String,
}

impl Event {
    fn from_file(path: impl AsRef<Path>) -> Event {
        let document = std::fs::read_to_string(path.as_ref()).expect("couldn't read event file");

        let arena = Arena::new();

        let options = comrak::Options {
            extension: ExtensionOptions::builder()
                .front_matter_delimiter(FRONT_MATTER_DELIMITER.to_owned())
                .build(),
            ..Default::default()
        };

        let root = parse_document(&arena, &document, &options);

        let front_matter_delim: Vec<_> = FRONT_MATTER_DELIMITER.chars().collect();

        // get raw front-matter blob
        let front_matter = root
            .descendants()
            .find_map(|node| match &node.data.borrow().value {
                NodeValue::FrontMatter(x) => Some(x.clone()),
                _ => None,
            })
            .expect("event document must have frontmatter");
        // trim '---' and newlines
        let front_matter = front_matter
            .trim()
            .trim_matches(front_matter_delim.as_slice())
            .trim();
        // parse
        let front_matter: EventFrontMatter =
            serde_yaml::from_str(front_matter).expect("invalid front-matter format");

        Event {
            title: front_matter.title.to_owned(),
            start: front_matter.start.to_owned(),
            end: front_matter.end.map(ToOwned::to_owned),
            content: {
                let mut buf = Vec::new();
                CustomFormatter::format_document(root, &options, &mut buf)
                    .expect("failed to format markdown");
                String::from_utf8_lossy(&buf).to_string()
            },
            sort_key: front_matter
                .sort_key
                .map(ToOwned::to_owned)
                .unwrap_or(front_matter.start.to_owned()),
        }
    }
}

create_formatter!(CustomFormatter, {
    NodeValue::Strong => |context, entering| {
        if entering {
            context.write_all(b"<span class=\"highlighted\"><span>")?;
        } else {
            context.write_all(b"</span><span class='highlight'></span><span class='shadow'></span></span>")?;
        }
    },
});

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventFrontMatter<'a> {
    title: &'a str,
    start: &'a str,
    end: Option<&'a str>,
    sort_key: Option<&'a str>,
}

pub fn load_events() -> Result<Box<[Event]>, ()> {
    let mut events = Vec::new();

    let read_dir = std::fs::read_dir(EVENTS_DIR).map_err(|_| ())?;
    for entry in read_dir {
        let Ok(entry) = entry else {
            continue;
        };

        let Ok(file_type) = entry.file_type() else {
            // couldn't get file type
            continue;
        };

        if !file_type.is_file() {
            // not a file
            continue;
        }

        let file_name = entry.file_name();
        let Some(file_name) = file_name.to_str() else {
            // filename isn't UTF-8 - should not happen
            continue;
        };

        if !file_name.ends_with(".md") {
            // only interested in md files
            continue;
        }

        events.push(Event::from_file(entry.path()));
    }

    events.sort_by_key(|event| event.sort_key.to_lowercase());

    Ok(events.into_boxed_slice())
}
