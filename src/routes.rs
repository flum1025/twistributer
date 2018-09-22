use std::io::Read;
use iron::status;
use iron::prelude::*;
use router::Router;
use serde_json::Value;
use crate::setting_loader::{Setting, User};
use crate::app;

pub fn init(router: &mut Router, setting: Setting) {
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

fn find_user(users: Vec<User>, body: String) -> Option<User> {
    let v: Value = serde_json::from_str(&body).unwrap();
    let user_id: &str = v["for_user_id"].as_str().unwrap();
    let user = users.into_iter().find(|user| user.user_id == user_id);

    return user;
}

fn generate_webhook(setting: Setting) -> impl Fn(&mut Request) -> IronResult<Response> {
    move |req| {
        let body = get_body(req);
        debug!("{:?} {:?}", req.url.to_string(), body);

        let user = find_user(setting.users.to_vec(), body.clone());
        println!("{:?}", user);


        if let Some(value) = user {
            app::delivery(value, body);
        }

        Ok(Response::with((status::Ok, "ok")))
    }
}
