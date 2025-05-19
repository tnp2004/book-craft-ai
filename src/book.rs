use serde::{Deserialize, Serialize};

use crate::{
    config::Config, image::GeminiClient
};

pub struct Book {
    config: Config,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BookContent {
    title: String,
    characters: Vec<Character>,
    story: Vec<Story>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Character {
    name: String,
    nature: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Story {
    content: String,
    image_prompt: String,
}

impl Book {
    pub fn new(config: Config) -> Self {
        return Book { config };
    }

    fn read_book_response(json_str: &str) -> Result<BookContent, serde_json::Error> {
        let book_content: BookContent = serde_json::from_str(json_str)?;

        Ok(book_content)
    }

    pub async fn create_book(&self, resp: &str) {
        let book_content = Self::read_book_response(resp).expect("Read book content failed");

        let gemini_client = GeminiClient::new(self.config.clone());
    }
}
