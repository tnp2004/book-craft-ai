use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub gemini_api: String
}

impl Config {
    pub fn read_config(path: &str) -> Self {
        let config_contents = fs::read_to_string(path).expect("Read config failed");
        let config: Config = toml::from_str(&config_contents).expect("Parse config failed");

        return config
    }
}