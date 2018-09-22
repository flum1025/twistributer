use std::fs;
use std::io::Read;
use yaml_rust::YamlLoader;
use std::collections::HashMap;

pub fn load(file: &str) -> HashMap<String, Vec<String>> {
    let string = file_read(file);
    let yaml = YamlLoader::load_from_str(&string.to_owned()).unwrap();
    let setting = &yaml[0];

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in setting.as_hash().unwrap().iter() {
        let mut ips: Vec<String> = Vec::new();
        for v in value.as_vec().unwrap() {
            ips.push(v.as_str().unwrap().to_string());
        }

        map.insert(key.as_i64().unwrap().to_string(), ips);
    }

    return map;
}

fn file_read(file: &str) -> String {
    let mut f = fs::File::open(file).unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();
    return std::str::from_utf8(&buf).unwrap().to_string();
}
