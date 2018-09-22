extern crate serde_json;

use std::io::Read;
use iron::status;
use iron::prelude::*;
use router::Router;
use std::collections::HashMap;
use serde_json::Value;

mod app;

pub fn init(router: &mut Router, setting: HashMap<String, Vec<String>>) {
    router.get("/", index, "index");
    router.post("/webhook", generate_webhook(setting), "webhook");
    router.get("/webhook", check_crc, "crc test");
    router.post("*", other_route, "404");
}

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "ok")))
}

fn get_body(req: &mut Request) -> String {
    let mut body = String::new();
    let _ = req.body.read_to_string(&mut body);
    return body;
}

fn other_route(req: &mut Request) -> IronResult<Response> {
    let body = get_body(req);
    debug!("{:?} {:?}", req.url.to_string(), body);

    Ok(Response::with((status::NotFound, "404 NotFound")))
}

fn check_crc(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "ok")))
}


fn generate_webhook(setting: HashMap<String, Vec<String>>) -> impl Fn(&mut Request) -> IronResult<Response> {
    move |req| {
        let body = get_body(req);
        let v: Value = serde_json::from_str(&body).unwrap();
        let user_id: &str = v["for_user_id"].as_str().unwrap();
        let endpoints = setting.get(user_id);

        debug!("{:?} {:?}", req.url.to_string(), body);

        if let Some(value) = endpoints {
            app::delivery(user_id.to_string(), body, value.to_vec());
        }

        Ok(Response::with((status::Ok, "ok")))
    }
}
