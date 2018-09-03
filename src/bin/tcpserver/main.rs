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

fn handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
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
                panic!(err);
            }
        }
    }
}

fn main() {
    let matches = App::new("clock-server")
        .version("1.0.0")
        .author("Sigurd Holsen")
        .about("Check if clock is in sync between computers")
        .arg(Arg::with_name("address").takes_value(true))
        .get_matches();

    let addr = if let Some(addr) = matches.value_of("address") {
        addr
    } else {
        "127.0.0.1:8080"
    };

    println!("Starting tcp server on {}", addr);
    let addr: SocketAddr = SocketAddr::from_str(addr).expect("Invalid address");
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}
