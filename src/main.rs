extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate byteorder;
extern crate chrono;

use chrono::Utc;

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
                .arg(
                    Arg::with_name("address")
                        .takes_value(true)
                        .index(1)
                        .help("NTP server and port"),
                ).arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Verbose output"),
                ),
        ).subcommand(
            SubCommand::with_name("client")
                .about("start client")
                .arg(
                    Arg::with_name("address")
                        .takes_value(true)
                        .index(1)
                        .help("Address with port to NTP server"),
                ).arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Verbose output"),
                ).arg(
                    Arg::with_name("repeat")
                        .takes_value(true)
                        .short("r")
                        .long("repeat")
                        .help("Number of requests to send. Performance testing only"),
                ),
        ).get_matches();

    let default_address = "127.0.0.1:8080";

    if let Some(client_matches) = matches.subcommand_matches("client") {
        let addr = client_matches
            .value_of("address")
            .unwrap_or(default_address);
        let is_verbose = client_matches.is_present("verbose");
        let repeat_count = client_matches
            .value_of("repeat")
            .map(|x| x.parse::<i32>().expect("Bad repeat count"))
            .unwrap_or(1);

        let now = Utc::now();
        let mut durations = vec![];
        for i in 0..repeat_count {
            if repeat_count > 1 {
                println!("Repeat: {}", i)
            }

            let dur = client::clock_diff_udp(addr, is_verbose).expect("Could not get clock diff");
            durations.push(dur);
        }
        if repeat_count > 1 {
            let dt = Utc::now() - now;
            let dt_ms = dt.num_milliseconds();
            println!("Doing {} requests took {}ms", repeat_count, dt_ms);

            let mut total_time = chrono::Duration::seconds(0);
            for dur in durations {
                total_time = total_time + dur;
            }

            let mean_time = total_time / repeat_count;
            println!("Mean time diff: {}ms", mean_time.num_milliseconds());
        }
    }

    if let Some(server_matches) = matches.subcommand_matches("server") {
        let addr = server_matches
            .value_of("address")
            .unwrap_or(default_address);
        let is_verbose = server_matches.is_present("verbose");

        server::run_server_udp(addr, is_verbose);
    }
}
