use book_craft_ai::{
    book, ollama::{OllamaClient, OllamaModel}, utils
};
use tokio;

#[tokio::main]
async fn main() {
    let prompt = utils::get_prompt();

    let ollama_client = OllamaClient::default();
    let res = ollama_client.send_question(OllamaModel::Gemma3, &prompt).await;

    book::read_book_response(&res);
}
