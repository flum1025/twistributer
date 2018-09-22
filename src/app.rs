use std::time::Duration;
use std::thread;
use std::thread::JoinHandle;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Response, Error};
use crate::setting_loader::User;

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

pub fn delivery(user: User, body: String) {
    let mut children: Vec<JoinHandle<()>> = vec![];

    for endpoint in user.endpoints {
        let cloned_name = user.name.clone();
        let cloned_body = body.clone();
        children.push(thread::spawn(move || {
            info!("{} : {}", cloned_name, endpoint);
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
