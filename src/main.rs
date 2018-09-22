#[macro_use]
extern crate log;
extern crate iron;
extern crate logger;
extern crate env_logger;
extern crate router;
extern crate yaml_rust;

use std::collections::HashMap;
use iron::prelude::*;
use logger::Logger;
use router::Router;

mod setting_loader;
mod routes;

const SETTING_FILE: &str = "setting.yml";

fn main() {
    env_logger::init();

    let setting = setting_loader::load(SETTING_FILE);
    start_server(setting);
}

fn start_server(setting: HashMap<String, Vec<String>>) {
    let mut router = Router::new();
    routes::init(&mut router, setting);

    let mut chain = Chain::new(router);

    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    match Iron::new(chain).http("localhost:3000") {
        Result::Ok(listening) => info!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}
