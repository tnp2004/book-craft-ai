use std::fs;

use serde::Deserialize;

pub struct File;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub gemini_api: String,
    pub image_dir: String,
}

#[derive(Deserialize, Debug)]
pub struct Instruction {
    pub instruction: String,
}

impl File {
    pub fn read_config(path: &str) -> Config {
        let contents = fs::read_to_string(path).expect("Read config failed");
        let config: Config = toml::from_str(&contents).expect("Parse config failed");

        return config;
    }

    pub fn read_instruction(path: &str) -> Instruction {
        let contents = fs::read_to_string(path).expect("Read instruction failed");
        let instruction: Instruction = serde_json::from_str(&contents).expect("Parse instruction failed");

        return instruction;
    }
}
