extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate byteorder;
extern crate chrono;
extern crate time;

mod client;
mod error;
mod ntp;
mod server;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("server")
                .about("start server")
                .arg(Arg::with_name("address").takes_value(true).index(1))
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Verbose output"),
                ),
        ).subcommand(
            SubCommand::with_name("client")
                .about("start client")
                .arg(Arg::with_name("address").takes_value(true).index(1)),
        ).get_matches();

    let default_address = "127.0.0.1:8080";

    if let Some(client_matches) = matches.subcommand_matches("client") {
        let addr = client_matches
            .value_of("address")
            .unwrap_or(default_address);
        client::clock_diff(addr);
    }

    if let Some(server_matches) = matches.subcommand_matches("server") {
        let addr = server_matches
            .value_of("address")
            .unwrap_or(default_address);
        let is_verbose = server_matches.is_present("verbose");

        server::run_server(addr, is_verbose);
    }
}
