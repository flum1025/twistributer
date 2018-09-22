extern crate reqwest;

use std::time::Duration;
use std::thread;
use std::thread::JoinHandle;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Response, Error};

const TIMEOUT: u64 = 5;

fn request(endpoint: String, body: String) -> std::result::Result<Response, Error> {
    return Client::builder()
        .timeout(Duration::from_secs(TIMEOUT))
        .build()
        .unwrap()
        .post(&endpoint)
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send();
}

pub fn delivery(user_id: String, body: String, endpoints: Vec<String>) {
    let mut children: Vec<JoinHandle<()>> = vec![];

    for endpoint in endpoints {
        let cloned_user_id = user_id.clone();
        let cloned_body = body.clone();
        children.push(thread::spawn(move || {
            info!("{} : {}", cloned_user_id, endpoint);
            let res = request(endpoint, cloned_body);

            match res {
                Err(e) => error!("{:?}", e.to_string()),
                Ok(_)  => return,
            }
        }));
    }

    // dont wait
    // for child in children {
    //     let _ = child.join();
    // }
}
