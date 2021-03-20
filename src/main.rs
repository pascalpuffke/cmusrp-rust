mod shell;
mod parser;

extern crate discord_rpc_client;

use discord_rpc_client::Client;
use std::{
    thread::sleep,
    time::Duration
};
use parser::Tag;

fn main() {
    const ID: u64 = 718109162923360327;
    const VERSION: &str = "1.0.0";

    let mut client = Client::new(ID);
    client.start();

    loop {
        let remote = shell::get_stdout("cmus-remote", "-Q");

        if parser::is_playing(&remote) {
            let artist = get_tag(Tag::Artist, &remote);
            let title = get_tag(Tag::Title, &remote);
            let album = get_tag(Tag::Album, &remote);
            let date = get_tag(Tag::Date, &remote);

            client.set_activity(|activity| {
                activity.state(format!("{} - {} ({})", artist, album, date))
                        .details(&title)
                        .assets(|asset| {
                            asset.large_image("icon").large_text(format!("version {}", VERSION))
                        })
            }).expect("Failed to set activity");
        } else {
            // don't show activity when playback is paused
            client.clear_activity().expect("Failed to clear activity");
        }

        sleep(Duration::from_secs(1));
    }
}

// basically just to get rid of repeated 'unwrap_or' in main method
fn get_tag(tag: parser::Tag, remote: &String) -> String {
    parser::parse_tag(tag, remote).unwrap_or("".to_string())
}
