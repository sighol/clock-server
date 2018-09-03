use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

extern crate chrono;
use clap::{App, Arg};

extern crate clap;
use chrono::prelude::*;

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
        "localhost:8080"
    };

    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Connected to {}", addr);

            let msg = [0; 10];

            let start = Utc::now();
            stream.write(&msg).unwrap();

            let mut data = [0; 4096]; // using 6 byte buffer
            match stream.read(&mut data) {
                Ok(n) => {
                    if true {
                        let end = Utc::now();
                        let read_slice = &data[0..n];
                        let mut vec = vec![];
                        vec.extend_from_slice(&read_slice);
                        let str = String::from_utf8(vec).expect("Not utf8");
                        let server_time = str.parse::<DateTime<Utc>>().expect("Bad time format");

                        compute_diff(start, end, server_time);
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
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

    let time_diff = real_server_time - receive_time;

    println!("time diff: {}", format_duration(time_diff));
}

fn format_duration(dur: chrono::Duration) -> String {
    let is_neg = dur < chrono::Duration::zero();
    let dur = if is_neg { -dur } else { dur };
    let dur = dur.to_std().unwrap();
    let secs = dur.as_secs() as f64 / 1_000.0;
    let nanos: f64 = dur.subsec_nanos() as f64 / 1_000_000.0;
    let result = secs + nanos;
    let sign = if is_neg { "-" } else { "" };
    format!("{}{}ms", sign, result)
}
