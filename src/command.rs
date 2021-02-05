use rhai;
use serde_json;
use std::fs;

pub fn load_json(path: &str) -> serde_json::Value {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    serde_json::from_str(&contents).unwrap()
}
