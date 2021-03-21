use std::fmt::Debug;
use std::{thread::sleep, time::Duration};

use discord_rpc_client::Client;
use structopt::StructOpt;

use parser::Tag;

mod parser;
mod shell;

// TODO: add support for custom format strings in rp display
#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(
        short,
        long,
        help = "Disables rich presence and displays current status on stdout"
    )]
    debug: bool,
    #[structopt(
        short,
        long,
        default_value = "1000",
        help = "Polling interval in which the program grabs the current status, in milliseconds"
    )]
    interval: u64,
}

const ID: u64 = 718109162923360327;
const VERSION: &str = "1.2.0";

fn main() {
    let args = Arguments::from_args();
    let mut client = Client::new(ID);

    // only start client if debug is disabled
    if !args.debug {
        client.start();
    }

    main_loop(client, args);
}

fn main_loop(mut client: Client, args_struct: Arguments) {
    loop {
        let remote = shell::get_stdout("cmus-remote", "-Q").unwrap_or(String::new());

        if parser::is_playing(&remote) {
            let (artist, title, album, date) = (
                get_tag(Tag::Artist, &remote),
                get_tag(Tag::Title, &remote),
                get_tag(Tag::Album, &remote),
                get_tag(Tag::Date, &remote),
            );

            if args_struct.debug {
                println!("{}\n{} - {} ({})\n", title, artist, album, date);
            } else {
                let state = if parser::is_tagged(&remote) {
                    format!("{} - {} ({})", artist, album, date)
                } else {
                    String::new()
                };

                client
                    .set_activity(|activity| {
                        activity.state(state).details(title).assets(|asset| {
                            asset
                                .large_image("icon")
                                .large_text(format!("version {}", VERSION))
                        })
                    })
                    .expect("Failed to set activity");
            }
        } else {
            // don't show activity when playback is paused
            client.clear_activity().expect("Failed to clear activity");
        }

        sleep(Duration::from_millis(args_struct.interval));
    }
}

// basically just to get rid of repeated unwrapping in main method
fn get_tag(tag: Tag, remote: &String) -> String {
    Tag::parse_tag(tag, remote).unwrap_or(String::new())
}
