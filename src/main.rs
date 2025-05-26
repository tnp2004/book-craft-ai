use book_craft_ai::{
    book::Book, config, file::File, ollama::{OllamaClient, OllamaModel}, utils
};
use tokio;

#[tokio::main]
async fn main() {
    let config = config::read_config("config.toml");
    let instruction = File::read_instruction(&config.directory.instruction);

    let prompt = utils::get_prompt();

    let ollama_client = OllamaClient::new(&config.ollama.host, config.ollama.port, instruction);
    let resp = match ollama_client.send_question(OllamaModel::Gemma3, &prompt).await {
        Ok(resp) => resp,
        Err(err) => panic!("{}", err)
    };

    let book = Book::new(config);
    let _ = book.create_book(&resp).await;
}
