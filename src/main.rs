use std::fmt::Debug;
use std::{thread::sleep, time::Duration};

use clap::{App, Arg};
use discord_rpc_client::Client;

use parser::Tag;

mod parser;
mod shell;

// TODO: add support for custom format strings in rp display
#[derive(Debug)]
struct Arguments {
    debug: bool,
    interval: u64,
}

const ID: u64 = 718109162923360327;
const VERSION: &str = "1.1.0";

fn main() {
    let mut args = Arguments {
        debug: false,
        interval: 1000,
    };
    parse_args(&mut args);

    let mut client = Client::new(ID);

    // only start client if debug is disabled
    if !args.debug {
        client.start();
    }

    main_loop(client, args);
}

fn parse_args(args_struct: &mut Arguments) {
    let args = App::new("cmusrp-rust")
        .version(VERSION)
        .author("Pascal Puffke <pascal@pascalpuffke.de>")
        .about("External Discord Rich Presence provider for cmus")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Disables rich presence and displays current status on stdout")
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .help("Polling interval in which the program grabs the current status, in milliseconds")
                .takes_value(true)
        )
        .get_matches();

    if args.is_present("interval") {
        let value = args
            .value_of("interval")
            .unwrap()
            .parse::<u64>()
            .unwrap_or_else(|e| panic!("Unable to parse integer from string: {}", e));

        args_struct.interval = value;
    }

    args_struct.debug = args.is_present("debug");
}

fn main_loop(mut client: Client, args_struct: Arguments) {
    loop {
        let remote = shell::get_stdout("cmus-remote", "-Q").unwrap_or(String::new());

        if parser::is_playing(&remote) {
            let artist = get_tag(Tag::Artist, &remote);
            let title = get_tag(Tag::Title, &remote);
            let album = get_tag(Tag::Album, &remote);
            let date = get_tag(Tag::Date, &remote);

            if args_struct.debug {
                println!("{}\n{} - {} ({})\n", title, artist, album, date);
            } else {
                client
                    .set_activity(|activity| {
                        activity
                            .state(format!("{} - {} ({})", artist, album, date))
                            .details(title)
                            .assets(|asset| {
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
