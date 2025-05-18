use serde::{Deserialize, Serialize};

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

pub fn read_book_response(json_str: &str) {
    let book_content: BookContent = serde_json::from_str(json_str).expect("Parse JSON failed");
    println!("title: {}", book_content.title);

    for character in book_content.characters {
        println!("character: {}", character.name)
    }

    for story in book_content.story {
        println!("content: {}", story.content)
    }
}
