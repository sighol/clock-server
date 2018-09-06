use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;

use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;

extern crate chrono;
use chrono::prelude::*;

use ntp;

pub fn run_server(addr: &str, is_verbose: bool) {
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

fn handle_client(mut stream: TcpStream, is_verbose: bool) {
    let source = stream
        .peer_addr()
        .map(|addr| format!("{}", addr))
        .unwrap_or(String::from("unknown"));
    if is_verbose {
        println!("Received connection from {}", source);
    }

    loop {
        let mut read = [0u8; 10000];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }

                let mut input = ntp::NTPHeader::decode(n, &read).expect("Could not decode NTP package");
                input.transmit_timestamp = ntp::NTPTimestamp::from_datetime(&Utc::now());

                let message = input.encode().expect("Could not encode NTP package");

                stream
                    .write(&message)
                    .expect("Could not write response");
            }
            Err(err) => {
                panic!("Failed to read client stream from {}. {}", source, err);
            }
        }
    }
}
