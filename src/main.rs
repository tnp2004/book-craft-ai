use std::env;

use book_craft_ai::{
    book::Book, config, file::File, ollama::{OllamaClient, OllamaModel}, utils
};
use tokio;

#[tokio::main]
async fn main() {
    let config = config::read_config("config.toml");
    let instruction = File::read_instruction(&config.directory.instruction);
    
    let prompt = utils::get_prompt();
    
    let loader = utils::create_loader("💬 Generating response");
    let ollama_client = OllamaClient::new(&config.ollama.host, config.ollama.port, instruction);
    let resp = match ollama_client.send_question(OllamaModel::Gemma3, &prompt).await {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err)
    };
    loader.success(" Generated response");

    loader.text("📘 Creating book");
    let book = Book::new(config);
    let book_dir = match book.create_book(&resp).await {
        Ok(dir) => dir,
        Err(err) => panic!("{}", err)
    };
    loader.end();
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => panic!("{}", err)
    };
    let book_path = format!("{}\\{}", current_dir.display(), book_dir.replace("/", "\\"));
    println!("📘 Book has been created at {}", book_path);
}
