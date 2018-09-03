#![feature(duration_as_u128)]
extern crate hyper;
extern crate pretty_env_logger;

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::SystemTime;

extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate reqwest;

extern crate humantime;

fn main() {
    pretty_env_logger::init();
    let matches = App::new("clock-server")
        .version("1.0.0")
        .author("Sigurd Holsen")
        .about("Check if clock is in sync between computers")
        .subcommand(
            SubCommand::with_name("server")
                .about("start server")
                .arg(Arg::with_name("addr").takes_value(true)),
        ).subcommand(
            SubCommand::with_name("client")
                .about("start client")
                .arg(Arg::with_name("ip").required(true).takes_value(true)),
        ).get_matches();

    if let Some(_server) = matches.subcommand_matches("server") {
        let addr = if let Some(conf_addr) = _server.value_of("addr") {
            SocketAddr::from_str(conf_addr).expect("Bad socket address")
        } else {
            ([0, 0, 0, 0], 3001).into()
        };
        start_server(addr);
    }

    if let Some(client) = matches.subcommand_matches("client") {
        let path = client.value_of("ip").unwrap();
        let start = SystemTime::now();
        let server_time_raw = request_server_time(path);
        let server_time = server_time_raw
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i128;
        let end = SystemTime::now();
        let request_duration = end.duration_since(start).unwrap().as_millis() as i128;

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i128;

        println!(
            "now: {}ms ",
            humantime::format_rfc3339_nanos(SystemTime::now())
        );
        println!(
            "server_time: {}ms",
            humantime::format_rfc3339_nanos(server_time_raw)
        );
        println!("request_duration: {}ms", request_duration);

        let real_server_time = server_time + request_duration / 2;
        let time_diff = now - real_server_time;
        println!("\nTime diff: {}ms", time_diff);
    }
}

fn request_server_time(path: &str) -> SystemTime {
    let mut body = reqwest::get(path).expect("No result");
    let text = body.text().expect("No text");
    let time = humantime::parse_rfc3339(&text).expect("Bad time format");
    time
}

fn start_server(addr: SocketAddr) {
    let new_service = || {
        service_fn_ok(|_| {
            let current_time = SystemTime::now();
            let format = humantime::format_rfc3339_nanos(current_time);
            let body = format!("{}", format);
            Response::new(Body::from(body))
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    get_current_time();
    rt::run(server);
}

fn get_current_time() -> u128 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_millis(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
