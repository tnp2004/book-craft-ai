use regex::Regex;
use reqwest::{Error, Response, StatusCode};

use crate::{
    config::Config, file::File, models::{GeminiRequest, GenerationConfig, Part, RequestContent}, utils
};

#[derive(Clone)]
pub struct GeminiClient {
    config: Config,
}

impl GeminiClient {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn generate_image(&self, prompt: &str, image_dir: &str) -> Result<String, Error> {
        let url = format!("{}?key={}", self.config.gemini.api_url, self.config.gemini.api_key);

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

        println!("Generating image . . .");

        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if resp.status() != StatusCode::OK {
            panic!("Generate image failed:{:?}", resp);
        }

        let file_name = self.create_image(resp, image_dir).await?;

        Ok(file_name)
    }

    async fn create_image(&self, response: Response, dir: &str) -> Result<String, Error> {
        let response_json = response.text().await?;

        let regex = Regex::new(r#""data": "([^"]*)""#).expect("Create Regex failed");

        let base64 = match regex.captures(&response_json) {
            Some(captures) => captures.get(1).map(|m| m.as_str()).unwrap_or(""),
            None => panic!("Could not find image data in response"),
        };

        let file_name = utils::generate_image_name("image");
        let image_path = format!("{}/{}", dir, file_name);

        if let Err(err) = File::create_file(base64, &image_path) {
            panic!("{}", err);
        }

        println!("{} has been created", file_name);

        Ok(file_name)
    }
}
