use std::io::{Read, Write};
use std::net::TcpStream;

extern crate chrono;
use chrono::prelude::*;

use ntp;

pub fn ntp_header() -> ntp::NTPHeader {
    let mut data = ntp::NTPHeader::new();
    data.origin_timestamp = ntp::NTPTimestamp::from_datetime(&Utc::now());
    data
}

pub fn clock_diff_udp(addr: &str, is_verbose: bool) -> Option<chrono::Duration> {
    use std::mem::size_of;
    use std::net::UdpSocket;

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not start udp socet");
    let send_time = Utc::now();
    let to_send = ntp_header();
    let msg = to_send.encode().expect("Bad encoded package");
    socket.send_to(&msg, addr).expect("Could not send");

    let mut bytes = [0u8; size_of::<ntp::NTPHeader>()];
    let n = socket.recv(&mut bytes).expect("Could not receive");
    let packet = ntp::NTPHeader::decode(n, &bytes).expect("Could not decode");
    let recv_time = Utc::now();
    if is_verbose {
        println!("Received NTP datagram: {:#?}", packet);
    }

    let dt = compute_diff(
        send_time,
        recv_time,
        packet.transmit_timestamp.to_datetime(),
    );

    Some(dt)
}

#[allow(dead_code)]
pub fn clock_diff_tcp(addr: &str) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Connected to {}", addr);

            let packet = ntp_header();

            let msg = packet.encode().expect("Encode package");

            let start = Utc::now();
            stream.write_all(&msg).unwrap();

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
) -> chrono::Duration {
    let duration = receive_time - send_time;
    println!("Send time:    {}", send_time);
    println!("server time:  {}", server_time);
    println!("receive time: {}", receive_time);

    println!("\ntransfer duration: {}", format_duration(duration));

    let real_server_time = server_time + duration / 2;

    let time_diff = receive_time - real_server_time;

    println!("\ntime diff: {}", format_duration(time_diff));
    time_diff
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
