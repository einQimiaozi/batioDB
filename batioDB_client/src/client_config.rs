extern crate serde_yaml;
extern crate serde;

use std::{fs, io};
use serde::{Serialize, Deserialize, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientConfig {
    pub port: String,
}

impl ClientConfig {
    pub fn new(config_path: &str) -> Self {
        let config_file = fs::File::open(config_path).expect("config file cannot find.");
        let client_config: ClientConfig = serde_yaml::from_reader(config_file)
            .expect("app.yaml read failed!");
        client_config
    }
    pub fn default() -> Self {
        ClientConfig {
            port: "127.0.0.1:8765".to_string(),
        }
    }
}