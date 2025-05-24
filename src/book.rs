use std::sync::Arc;

use futures::future;
use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::{config::Config, file::File, image::GeminiClient, utils};

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

        let images = match self.generate_book_image(book_content).await {
            Ok(images) => images,
            Err(err) => panic!("{}", err)
        };
    }

    async fn generate_book_image(&self, book_content: BookContent) -> Result<Vec<String>, Error> {
        let gemini_client = Arc::new(GeminiClient::new(self.config.clone()));

        let title = utils::create_dir_name(book_content.title);
        let image_dir = format!("{}/{}", self.config.directory.image, title);
        if let Err(err) = File::create_directory(&image_dir) {
            panic!("{}", err)
        };

        let tasks: Vec<_> = book_content
            .story
            .into_iter()
            .map(|story| {
                let client = Arc::clone(&gemini_client);
                let image_theme = book_content.image_theme.clone();
                let dir = image_dir.clone();
                tokio::spawn(async move {
                    let prompt = format!("{}\n{}", image_theme, story.image_prompt);
                    client
                        .generate_image(&prompt, &dir)
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

        Ok(file_name_vec)
    }
}
