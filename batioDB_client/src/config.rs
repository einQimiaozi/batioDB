extern crate serde_yaml;
extern crate serde;

use std::{fs, io};
use serde::{Serialize, Deserialize, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct DBConfig {
    pub port: String,
}

impl DBConfig {
    pub fn new(config_path: &str) -> Self {
        let config_file = fs::File::open(config_path).expect("config file cannot find.");
        let db_config: DBConfig = serde_yaml::from_reader(config_file)
            .expect("app.yaml read failed!");
        db_config
    }
    pub fn default() -> Self {
        DBConfig {
            port: "127.0.0.1:8765".to_string(),
        }
    }
}