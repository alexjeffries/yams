/* yams - yet another mock server
 *
 * Copyright (C) 2017 Alex Jeffries
 *
 * See LICENSE
 *
 */

#[macro_use]
extern crate clap;
extern crate hyper;
extern crate futures;

use clap::{App, ArgMatches};
use hyper::server::{Http, Request, Response, Service};
use std::fs::File;
use std::io::prelude::*;
use std::net::IpAddr;

mod lib;

static BANNER: &'static str = r#"
 _   _  __ _ _ __ ___  ___
| | | |/ _` | '_ ` _ \/ __|
| |_| | (_| | | | | | \__ \  -- yet another mock server
 \__, |\__,_|_| |_| |_|___/
 |___/
"#;

fn main() -> () {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = parse_matches(&matches);
    run(config).unwrap();
}

fn parse_matches<'a>(matches: &'a ArgMatches) -> AppConfig<'a> {
    let config_filename = matches.value_of("configuration_file")
        .unwrap();

    let addr: IpAddr = matches.value_of("address")
        .unwrap()
        .parse()
        .expect("Could not parse given address");

    let port: u16 = matches.value_of("port")
        .unwrap()
        .parse()
        .expect("Expected a number for the port");

    AppConfig {
        config_filename: config_filename,
        addr: addr,
        port: port,
    }
}

fn run<'a>(app_config: AppConfig) -> Result<(), hyper::Error> {
    let mut f = File::open(app_config.config_filename).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    println!("{}", BANNER);

    let addr = (app_config.addr, app_config.port).into();
    let mocks: Vec<lib::Mock> = lib::load_mocks(&contents);


    let server = Http::new().bind(&addr, move || Ok(YamsService { mocks: &mocks }))?;

    let result = server.run();

    result
}

struct AppConfig<'a> {
    config_filename: &'a str,
    addr: IpAddr,
    port: u16,
}

struct YamsService<'a> {
    mocks: &'a Vec<lib::Mock>,
}

impl<'a> Service for YamsService<'a> {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, request: Request) -> Self::Future {
        futures::future::ok(match find_mock_result(&self.mocks, request) {
            Ok(mock_response) => {
                Response::new()
                    .with_status(mock_response.status())
                    .with_headers(mock_response.headers())
                    .with_body(mock_response.body())
            }
            Err(error) => {
                Response::new()
                    .with_status(hyper::StatusCode::InternalServerError)
                    .with_body(error)
            }
        })
    }
}


fn find_mock_result(mocks: &Vec<lib::Mock>, request: Request) -> Result<&lib::Response, String> {
    let mut result: Result<&lib::Response, String> = Err(String::from("no matching mock found"));

    for mock in mocks {
        if mock.matches(&request) {
            result = Ok(mock.response());
            break;
        }
    }

    result
}
