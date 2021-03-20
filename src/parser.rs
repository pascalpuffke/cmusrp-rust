pub enum Tag {
    Title,
    Album,
    Artist,
    Date
}

pub fn is_playing(remote: &String) -> bool {
    remote.contains("status playing")
}

pub fn parse_tag(tag: Tag, remote: &String) -> Option<String> {
    // getting proper string to later search for based on the Tag enum
    let tag_formatted = format!("tag {} ", match tag {
        Tag::Title => "title",
        Tag::Album => "album",
        Tag::Artist => "artist",
        Tag::Date => "date"
    });

    for line in remote.lines() {
        if line.starts_with(tag_formatted.as_str()) {
            // return statement is required here for some odd reason
            return Some(line.chars().skip(tag_formatted.len()).collect());
        }
    }

    None
}
