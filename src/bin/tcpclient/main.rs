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

    let real_server_time = server_time + duration / 2;

    let time_diff = real_server_time - receive_time;
    println!("time diff: {}ms", time_diff.num_milliseconds());
}

// pub fn compute_diff(send_time: SystemTime, receive_time: SystemTime, server_time: SystemTime) {
//     let request_duration_ms = receive_time.duration_since(send_time).unwrap().as_millis() as i128;
//     let receive_time_ms = receive_time
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap()
//         .as_millis() as i128;

//     println!("now: {}ms ", humantime::format_rfc3339_nanos(receive_time));
//     println!(
//         "server_time: {}ms",
//         humantime::format_rfc3339_nanos(server_time)
//     );

//     let server_time_ms = server_time
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap()
//         .as_millis() as i128;

//     println!("request_duration_ms: {}ms", request_duration_ms);

//     let real_server_time = server_time_ms + request_duration_ms / 2;
//     let time_diff = receive_time_ms - real_server_time;
//     println!("\nTime diff: {}±{}ms", time_diff, request_duration_ms);
// }
