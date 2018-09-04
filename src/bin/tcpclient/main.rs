use clap::{App, Arg};

extern crate clap;

extern crate clock_server;
use clock_server::client;

use std::thread;
use std::time::Duration;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("address")
                .takes_value(true)
                .index(1)
                .help("TCP address to server"),
        )
        .arg(Arg::with_name("test").long("--test"))
        .get_matches();

    let addr = matches.value_of("address").unwrap_or("127.0.0.1:8080");

    if matches.is_present("test") {
        let mut i = 0;
        loop {
            i += 1;
            println!("Test: {}", i);
            client::clock_diff(addr);
            thread::sleep(Duration::from_millis(1));
        }
    } else {
        client::clock_diff(addr);
    }
}
