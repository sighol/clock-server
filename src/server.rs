use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;

use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;

extern crate chrono;
use chrono::prelude::*;

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
