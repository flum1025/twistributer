use std::io::Read;
use iron::status;
use iron::prelude::*;
use router::Router;
use serde_json::Value;
use crate::setting_loader::{Setting, User, ApiKey};
use crate::app;
use url::Url;
use std::collections::HashMap;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;

pub fn init(router: &mut Router, setting: Setting) {
    router.get("/", index, "index");
    router.post("/webhook", generate_webhook(setting.clone()), "webhook");
    router.get("/webhook", generate_crc_check(setting.app.api_key), "crc check");
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

#[derive(Serialize, Deserialize)]
struct Signature {
    response_token: String,
}

fn generate_signature(secret: String, crc_token: String) -> String {
    let mut hmac = Hmac::new(Sha256::new(), secret.as_bytes());
    hmac.input(crc_token.as_bytes());
    let result = hmac.result();

    let signature = Signature {
        response_token: format!("sha256={}", base64::encode(&result.code().to_vec())),
    };

    return serde_json::to_string(&signature).unwrap();
}

fn generate_crc_check(api_key: ApiKey) -> impl Fn(&mut Request) -> IronResult<Response> {
    move |req| {
        let url = Url::parse(&req.url.to_string()).unwrap();
        let query: HashMap<_, _> = url.query_pairs().into_owned().collect();

        match query.get("crc_token") {
            Some(value) => Ok(Response::with((status::Ok, generate_signature(api_key.consumer_secret.to_string(), value.to_string())))),
            _ => Ok(Response::with((status::Ok, "ok"))),
        }
    }
}

