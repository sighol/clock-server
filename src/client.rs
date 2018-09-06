use std::io::{Read, Write};
use std::net::TcpStream;

extern crate chrono;
use chrono::prelude::*;

use ntp;

pub fn ntp_header() -> ntp::NTPHeader {
    let mut data = ntp::NTPHeader::new();
    data.stratum(3);
    data.origin_timestamp = ntp::NTPTimestamp::from_datetime(&Utc::now());
    data
}

pub fn clock_diff(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Connected to {}", addr);

            let packet = ntp_header();

            let msg = packet.encode().expect("Encode package");

            let start = Utc::now();
            stream.write(&msg).unwrap();

            let mut data = [0; 4096]; // using 6 byte buffer
            match stream.read(&mut data) {
                Ok(n) => {
                    let end = Utc::now();
                    
                    let packet = ntp::NTPHeader::decode(n, &data).expect("Invalid server time");
                    let server_time = packet.transmit_timestamp.to_datetime();

                    compute_diff(start, end, server_time);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

pub fn compute_diff(
    send_time: DateTime<Utc>,
    receive_time: DateTime<Utc>,
    server_time: DateTime<Utc>,
) {
    let duration = receive_time - send_time;
    println!("now:         {}", receive_time);
    println!("server time: {}", server_time);
    println!("send time: {}", format_duration(duration));

    let real_server_time = server_time + duration / 2;

    let time_diff = receive_time - real_server_time;

    println!("time diff: {}", format_duration(time_diff));
}

fn format_duration(dur: chrono::Duration) -> String {
    let is_neg = dur < chrono::Duration::zero();
    let (dur, sign) = if is_neg { (-dur, "-") } else { (dur, "") };
    let dur = dur.to_std().unwrap();
    let ms = dur.as_secs() as f64 * 1_000.0;
    let nanos: f64 = dur.subsec_nanos() as f64 / 1_000_000.0;
    let result = ms + nanos;
    format!("{}{}ms", sign, result)
}
