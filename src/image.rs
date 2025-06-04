use anyhow::{Result, anyhow};
use regex::Regex;
use reqwest::{Response, StatusCode};

use crate::{
    config::Config,
    file::File,
    models::{GeminiRequest, GenerationConfig, Part, RequestContent},
    utils,
};

#[derive(Clone)]
pub struct GeminiClient {
    config: Config,
}

impl GeminiClient {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn generate_image(&self, prompt: &str, image_dir: &str) -> Result<String> {
        let url = format!(
            "{}?key={}",
            self.config.gemini.api_url, self.config.gemini.api_key
        );

        let request_body = GeminiRequest {
            contents: vec![RequestContent {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
            generation_config: GenerationConfig {
                response_modalities: vec!["TEXT".to_string(), "IMAGE".to_string()],
            },
        };

        let client = reqwest::Client::new();

        let resp = client.post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if resp.status() != StatusCode::OK {
            return Err(anyhow!("Generate image failed:{:?}", resp));
        }

        let file_name = self.create_image(resp, image_dir).await?;

        Ok(file_name)
    }

    async fn create_image(&self, response: Response, dir: &str) -> Result<String> {
        let response_json = response.text().await?;

        let regex = Regex::new(r#""data": "([^"]*)""#)?;

        let base64 = match regex.captures(&response_json) {
            Some(captures) => captures.get(1).map(|m| m.as_str()).unwrap(),
            None => return Err(anyhow!("Could not find image data in response")),
        };

        let file_name = utils::generate_image_name("image")?;
        let image_path = format!("{}/{}", dir, file_name);

        File::create_file(base64, &image_path)?;

        Ok(file_name)
    }
}
