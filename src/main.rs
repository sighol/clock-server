#![feature(duration_as_u128)]
extern crate hyper;
extern crate pretty_env_logger;

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use std::net::SocketAddr;
use std::time::SystemTime;

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
                .arg(Arg::with_name("ip")),
        ).get_matches();

    println!("Matches: {:?}", matches);

    if let Some(_server) = matches.subcommand_matches("server") {
        let addr = ([127, 0, 0, 1], 3001).into();
        start_server(addr);
    }

    if let Some(_client) = matches.subcommand_matches("client") {
        let r = reqwest::get("https://www.rust-lang.org");
        println!("r: {:?}", r);
        // let body = r?.text()?;
        // println!("Received response: {}", body);
    }
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
