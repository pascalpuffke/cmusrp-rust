use std::{fmt::Debug, thread::sleep, time::Duration};

use discord_rpc_client::Client;
use structopt::StructOpt;

use parser::Tag;

mod parser;
mod shell;

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
    #[structopt(
        short,
        long = "top",
        default_value = "{title}",
        help = "Sets custom formatting for the top string"
    )]
    top_format: String,
    #[structopt(
        short,
        long = "bottom",
        default_value = "{artist} - {album} ({date})",
        help = "Sets custom formatting for the bottom string"
    )]
    bottom_format: String,
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
            let (top, bottom) = (
                replace_format(&args_struct.top_format, &remote),
                replace_format(&args_struct.bottom_format, &remote),
            );

            if args_struct.debug {
                println!("{}\n{}", top, bottom);
            } else {
                client
                    .set_activity(|activity| {
                        activity.state(bottom).details(top).assets(|asset| {
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

fn replace_format(format_string: &str, remote: &str) -> String {
    format_string
        .replace("{artist}", get_tag(Tag::Artist, &remote).as_str())
        .replace("{title}", get_tag(Tag::Title, &remote).as_str())
        .replace("{album}", get_tag(Tag::Album, &remote).as_str())
        .replace("{date}", get_tag(Tag::Date, &remote).as_str())
}

// basically just to get rid of repeated unwrapping in main method
fn get_tag(tag: Tag, remote: &str) -> String {
    Tag::parse_tag(tag, remote).unwrap_or(String::new())
}
