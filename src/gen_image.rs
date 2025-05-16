use std::{fs::File, io::Write};

use base64::{Engine, engine::general_purpose};
use regex::Regex;
use reqwest::{Error, Response};

use crate::{
    models::{GeminiRequest, GenerationConfig, Part, RequestContent},
    utils,
};

pub struct GeminiClient {
    api_key: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn generate_image(&self, prompt: &str) -> Result<(), Error> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-preview-image-generation:generateContent?key={}",
            self.api_key
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

        println!("Generating . . .");
        
        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        self.create_image(resp).await?;

        Ok(())
    }

    async fn create_image(&self, response: Response) -> Result<(), Error> {
        let response_json = response.text().await?;

        let regex = Regex::new(r#""data": "([^"]*)""#).expect("Create Regex failed");

        let base64 = match regex.captures(&response_json) {
            Some(captures) => captures.get(1).map(|m| m.as_str()).unwrap_or(""),
            None => panic!("Could not find image data in response"),
        };

        let file_name = utils::generate_image_name("image");
        let image_path = format!("images/{}", file_name);

        let image_data = general_purpose::STANDARD
            .decode(base64)
            .expect("Decode base64 image data failed");
        let mut file = File::create(image_path).expect("Create file failed");
        file.write_all(&image_data)
            .expect("Write file buffer failed");

        println!("{} has been created", file_name);

        Ok(())
    }
}
