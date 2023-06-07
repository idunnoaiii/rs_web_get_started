use serde_yaml;
use std::collections::HashMap;
use std::env::{self, args};

pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>,
}

impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        dbg!("{}", &args);
        let file_path = &args[args.len() - 1];
        let file = std::fs::File::open(file_path).unwrap();
        let map = serde_yaml::from_reader(file).unwrap();
        return Config { map }
    }
}
