use std::sync::Arc;

use futures::future;
use serde::{Deserialize, Serialize};

use crate::{config::Config, image::GeminiClient};

pub struct Book {
    config: Config,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BookContent {
    title: String,
    characters: Vec<Character>,
    image_theme: String,
    story: Vec<Story>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Character {
    name: String,
    nature: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

        let gemini_client = Arc::new(GeminiClient::new(self.config.clone()));

        let tasks: Vec<_> = book_content
            .story
            .into_iter()
            .map(|story| {
                let client = Arc::clone(&gemini_client);
                let image_theme = book_content.image_theme.clone();
                tokio::spawn(async move {
                    let prompt = format!("{}\n{}", image_theme, story.image_prompt);
                    client
                        .generate_image(&prompt)
                        .await
                        .unwrap_or_else(|err| panic!("Image generation failed: {}", err))
                })
            })
            .collect();

        let file_name_vec: Vec<String> = future::join_all(tasks)
            .await
            .into_iter()
            .map(|res| res.expect("Task panicked"))
            .collect();

        println!("{:?}", file_name_vec);
    }
}
