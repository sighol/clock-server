#![feature(duration_as_u128)]

extern crate humantime;
use std::time::SystemTime;

pub fn compute_diff(send_time: SystemTime, receive_time: SystemTime, server_time: SystemTime) {
    let request_duration_ms = receive_time.duration_since(send_time).unwrap().as_millis() as i128;
    let receive_time_ms = receive_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i128;

    println!("now: {}ms ", humantime::format_rfc3339_nanos(receive_time));
    println!(
        "server_time: {}ms",
        humantime::format_rfc3339_nanos(server_time)
    );

    let server_time_ms = server_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i128;

    println!("request_duration_ms: {}ms", request_duration_ms);

    let real_server_time = server_time_ms + request_duration_ms / 2;
    let time_diff = receive_time_ms - real_server_time;
    println!("\nTime diff: {}±{}ms", time_diff, request_duration_ms);
}
