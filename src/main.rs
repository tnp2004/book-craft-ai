use book_craft_ai::ollama::{OllamaClient, OllamaModel};
use tokio;

#[tokio::main]
async fn main() {
    let prompt = "Write some story about the boy who is advanturer that use sword as a weapon in the Fantasy world.".to_string();

    let ollama_client = OllamaClient::default();
    let res = match ollama_client.send_question(OllamaModel::Gemma3, &prompt).await {
        Ok(res) => res,
        Err(err) => panic!("Ollama error: {}", err),
    };

    println!("Question: {}\nResponse by {}\n\n{}", prompt, OllamaModel::Gemma3, res.response);
}
