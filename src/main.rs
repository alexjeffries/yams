/* yams - yet another mock server
 *
 * Copyright (C) 2017 Alex Jeffries
 *
 * See LICENSE
 *
 */

extern crate hyper;
extern crate futures;

use hyper::server::{Http, Request, Response, Service};
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process;

mod lib;

static BANNER: &'static str = r#"
 _   _  __ _ _ __ ___  ___
| | | |/ _` | '_ ` _ \/ __|
| |_| | (_| | | | | | \__ \  -- yet another mock server
 \__, |\__,_|_| |_| |_|___/
 |___/
"#;

static USAGE: &'static str = r#"
Usage: yams CONFIG_FILENAME
"#;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    if args.len() != 2 {
        writeln!(&mut stderr, "Please provide a filename\n{}", USAGE)
            .expect("Could not write to stderr");

        process::exit(1);
    }

    let filename = &args[1];
    run(filename).unwrap();
}

fn run<'a>(filename: &str) -> Result<(), hyper::Error> {
    let mut f = File::open(filename).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    println!("{}", BANNER);

    let addr = ([127, 0, 0, 1], 3333).into();
    let mocks: Vec<lib::Mock> = lib::load_mocks(&contents);


    let server = Http::new().bind(&addr, move || Ok(YamsService { mocks: &mocks }))?;

    let result = server.run();

    result
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
