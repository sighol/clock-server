use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;

use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;

extern crate chrono;
use chrono::prelude::*;

use ntp;

pub fn run_server_udp(addr: &str, is_verbose: bool) {
    use std::mem::size_of;
    use std::net::UdpSocket;
    let socket = UdpSocket::bind(addr).expect("Could not bind to UDP socket");

    println!("Starting NTP server on {}", addr);

    loop {
        let mut buffer = [0u8; size_of::<ntp::NTPHeader>()];
        match socket.recv_from(&mut buffer) {
            Ok((n, addr)) => {
                let mut packet = ntp::NTPHeader::decode(n, &buffer).expect("Could not decode");
                packet.receive_timestamp = ntp::NTPTimestamp::from_datetime(&Utc::now());
                packet.transmit_timestamp = ntp::NTPTimestamp::from_datetime(&Utc::now());
                packet.stratum = 8;
                let msg = packet.encode().expect("Could not encode");
                socket.send_to(&msg, addr).expect("Could not send");
                if is_verbose {
                    println!("Received datagram from {}:\n{:#?}", addr, packet);
                }
            }
            Err(e) => {
                println!("Failed to receive: {}", e);
            }
        }
    }
}

#[allow(dead_code)]
pub fn run_server_tcp(addr: &str, is_verbose: bool) {
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

                let mut input =
                    ntp::NTPHeader::decode(n, &read).expect("Could not decode NTP package");
                input.transmit_timestamp = ntp::NTPTimestamp::from_datetime(&Utc::now());

                let message = input.encode().expect("Could not encode NTP package");

                stream.write_all(&message).expect("Could not write response");
            }
            Err(err) => {
                panic!("Failed to read client stream from {}. {}", source, err);
            }
        }
    }
}
