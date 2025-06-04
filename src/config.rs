use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub directory: Directory,
    pub gemini: Gemini,
    pub ollama: Ollama,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Gemini {
    pub api_key: String,
    pub api_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Directory {
    pub books: String,
    pub instruction: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ollama {
    pub host: String,
    pub port: u16,
}

pub fn read_config(path: &str) -> Result<Config> {
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}
