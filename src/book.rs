use std::sync::Arc;

use futures::future;
use minify_html::{minify, Cfg};
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

    pub async fn create_book(&self, resp: &str) -> Result<String, serde_json::Error> {
        let book_content = Self::read_book_response(resp)?;

        let images_path = match self.generate_book_image(book_content.clone()).await {
            Ok(images) => images,
            Err(err) => panic!("{}", err)
        };

        let mut story_elems = String::new();

        for (i, story) in book_content.story.iter().enumerate() {
            let image = images_path.get(i).expect("Image not found");
            let elem = format!(r#"
                <div>
                    <img class="shadow-sm mb-3 border-4 border-double object-cover" src="{}" alt="{}">
                    <p>{}</p>
                </div>
            "#, image, book_content.title, story.content);

            story_elems.push_str(&elem);
        }

        let html = format!(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
            <link rel="preconnect" href="https://fonts.googleapis.com">
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
            <link href="https://fonts.googleapis.com/css2?family=Delius&display=swap" rel="stylesheet">
            <title>{}</title>
        </head>
        <body class="font-[Delius]">
            <div class="bg-amber-50 w-[700px] m-auto my-5 p-5 drop-shadow-md">
                <h1 class="text-center text-2xl font-bold mb-5">{}</h1>
                <div class="flex flex-col gap-5">
                    {}
                </div>
            </div>
        </body>
        </html>
        "#, book_content.title, book_content.title, story_elems);

        let minified_html = minify(html.as_bytes(), &Cfg::new());

        let title = utils::create_dir_name(book_content.title.clone());
        let book_dir = format!("{}/{}/{}.html", self.config.directory.books, title, title);
        if let Err(err) = File::create_html(&minified_html, &book_dir) {
            panic!("{}", err)
        }

        Ok(book_dir)
    }

    async fn generate_book_image(&self, book_content: BookContent) -> Result<Vec<String>, Error> {
        let gemini_client = Arc::new(GeminiClient::new(self.config.clone()));

        let title = utils::create_dir_name(book_content.title);
        let image_dir = format!("{}/{}/{}", self.config.directory.books, title, "images");
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
        .map(|res| {
            let path = res.expect("Task panicked");
            
            format!("{}/{}", "images", path)
        })
        .collect();
    
        Ok(file_name_vec)
    }
}
