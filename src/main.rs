#![feature(duration_as_u128)]
extern crate hyper;
extern crate pretty_env_logger;

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};
use std::thread;

extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate reqwest;

fn main() {
    pretty_env_logger::init();
    let matches = App::new("clock-server")
        .version("1.0.0")
        .author("Sigurd Holsen")
        .about("Check if clock is in sync between computers")
        .subcommand(SubCommand::with_name("server").about("start server"))
        .subcommand(
            SubCommand::with_name("client")
                .about("start client")
                .arg(Arg::with_name("ip").required(true).takes_value(true)),
        ).get_matches();

    if let Some(_server) = matches.subcommand_matches("server") {
        let addr = ([127, 0, 0, 1], 3001).into();
        start_server(addr);
    }

    if let Some(client) = matches.subcommand_matches("client") {
        let path = client.value_of("ip").unwrap();
        let start = SystemTime::now();
        let server_time = request_server_time(path)
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_millis() as i128;
        let end = SystemTime::now();
        let requst_duration = end.duration_since(start).unwrap().as_millis() as i128;

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_millis() as i128;

        let real_server_time = server_time + requst_duration/2;
        let time_diff = now - real_server_time;
        println!("Time diff: {}ms", time_diff);
    }
}

fn request_server_time(path: &str) -> SystemTime {
    let mut body = reqwest::get(path).expect("No result");
    let text = body.text().expect("No text");
    let millis =
        u64::from_str_radix(&text, 10).expect("Bad response from server. Not clock sync server");
    println!("Text: {}", millis);
    let duration_since_epoch = Duration::from_millis(millis);
    SystemTime::UNIX_EPOCH + duration_since_epoch
}

fn start_server(addr: SocketAddr) {
    let new_service = || {
        service_fn_ok(|_| {
            let body = format!("{}", get_current_time());
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
