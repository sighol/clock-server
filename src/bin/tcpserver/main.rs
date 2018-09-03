use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;

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
    let addr: SocketAddr = SocketAddr::from_str("127.0.0.1:8080").expect("Invalid address");
    println!("Starting tcp server on {}", addr);
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
