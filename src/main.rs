#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate iron;
extern crate logger;
extern crate env_logger;
extern crate router;
extern crate serde_yaml;
extern crate serde_json;
extern crate yaml_rust;
extern crate reqwest;
extern crate url;
extern crate crypto;
extern crate base64;

mod setting_loader;
mod routes;
mod app;

use iron::prelude::*;
use logger::Logger;
use router::Router;
use self::setting_loader::Setting;

const SETTING_FILE: &str = "setting.yml";

fn main() {
    env_logger::init();

    let setting = setting_loader::load(SETTING_FILE);
    start_server(setting);
}

fn start_server(setting: Setting) {
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
