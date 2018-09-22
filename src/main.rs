extern crate iron;
extern crate router;
extern crate yaml_rust;

use std::fs;
use std::io::Read;
use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use router::Router;
use yaml_rust::YamlLoader;

const SETTING_FILE: &str = "setting.yml";

fn main() {
    let setting = setting_loader();
    start_server(setting);
}

fn setting_loader() -> HashMap<String, Vec<String>> {
    let string = file_read();
    let yaml = YamlLoader::load_from_str(&string.to_owned()).unwrap();
    let setting = &yaml[0];

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in setting.as_hash().unwrap().iter() {
        let mut ips: Vec<String> = Vec::new();
        for v in value.as_vec().unwrap() {
            ips.push(v.as_str().unwrap().to_string());
        }

        map.insert(key.as_str().unwrap().to_string(), ips);
    }

    return map;
}

fn file_read() -> String {
    let mut f = fs::File::open(SETTING_FILE).unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();
    return std::str::from_utf8(&buf).unwrap().to_string();
}

fn start_server(setting: HashMap<String, Vec<String>>) {
    let mut router = Router::new();
    router.get("/", index, "index");

    Iron::new(router).http("localhost:3000").unwrap();

    fn index(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }
}
