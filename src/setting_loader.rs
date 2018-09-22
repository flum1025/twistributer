use std::fs;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    pub api_key: ApiKey,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub user_id: String,
    pub endpoints: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Setting {
    pub app: App,
    pub users: Vec<User>,
}

pub fn load(file: &str) -> Setting {
    let data = file_read(file);
    let setting: Setting = serde_yaml::from_str(&data).unwrap();

    return setting;
}

fn file_read(file: &str) -> String {
    let mut f = fs::File::open(file).unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();
    return std::str::from_utf8(&buf).unwrap().to_string();
}
