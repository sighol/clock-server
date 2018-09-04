extern crate clap;
use clap::{App, Arg};

extern crate clock_server;
use clock_server::server;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("address")
                .takes_value(true)
                .index(1)
                .help("TCP address to client"),
        )
        .arg(Arg::with_name("verbose").short("v").long("verbose"))
        .get_matches();

    let addr = matches.value_of("address").unwrap_or("127.0.0.1:8080");
    let is_verbose = matches.is_present("verbose");
    println!("Starting tcp server on {}", addr);

    server::run_server(addr, is_verbose);
}
