use core::fmt;
use std::fmt::{Display, Formatter};

pub enum Tag {
    Title,
    Album,
    Artist,
    Date,
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Tag::Title => "title",
                Tag::Album => "album",
                Tag::Artist => "artist",
                Tag::Date => "date",
            }
        )
    }
}

impl Tag {
    pub fn parse_tag(tag: Tag, remote: &str) -> Option<String> {
        // getting proper string to later search for based on the Tag enum
        let tag_formatted = format!("tag {} ", tag.to_string());

        let results: Vec<&str> = remote
            .lines()
            .filter(|line| line.starts_with(tag_formatted.as_str()))
            .collect();

        if results.len() > 0 {
            Some(results.get(0).unwrap()[tag_formatted.len()..].to_string())
        } else {
            None
        }
    }
}

pub fn is_playing(remote: &str) -> bool {
    remote.contains("status playing")
}
