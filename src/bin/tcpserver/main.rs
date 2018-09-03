use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;

extern crate clap;
use clap::{App, Arg};

extern crate chrono;
use chrono::prelude::*;

fn handle_client(mut stream: TcpStream, is_verbose: bool) {
    let source = stream
        .peer_addr()
        .map(|addr| format!("{}", addr))
        .unwrap_or(String::from("unknown"));
    if is_verbose {
        println!("Received connection from {}", source);
    }

    loop {
        let mut read = [0; 10];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }

                let message = format!("{}", Utc::now().to_rfc3339());

                let message_bin = message.as_bytes();
                stream
                    .write(&message_bin)
                    .expect("Could not write response");
            }
            Err(err) => {
                panic!("Failed to read client stream from {}. {}", source, err);
            }
        }
    }
}

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
    let addr: SocketAddr = SocketAddr::from_str(addr).expect("Invalid address");
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream, is_verbose);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}
