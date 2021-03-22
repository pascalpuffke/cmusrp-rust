use std::{
    fmt,
    fmt::{Display, Formatter},
};

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

        if is_tagged(remote) {
            return Some(
                remote
                    .lines()
                    .filter(|line| line.starts_with(tag_formatted.as_str()))
                    .take(1)
                    .collect::<String>()[tag_formatted.len()..]
                    .to_string(),
            );
        }

        if tag.to_string().eq("title") {
            // return only the title tag based on file name
            return Some(
                remote
                    .lines()
                    .filter(|line| line.starts_with("file "))
                    .take(1)
                    .collect::<String>()[5..]
                    .to_string(),
            );
        }

        None
    }
}

pub fn is_tagged(remote: &str) -> bool {
    remote.contains("tag title")
}

pub fn is_playing(remote: &str) -> bool {
    remote.contains("status playing")
}
