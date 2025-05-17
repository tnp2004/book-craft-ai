use book_craft_ai::{
    file::File,
    image::GeminiClient,
    ollama::{OllamaClient, OllamaModel},
    utils,
};
use tokio;

#[tokio::main]
async fn main() {
    let config = File::read_config("config.toml");
  
    let prompt = utils::get_prompt();
    let gemini_client = GeminiClient::new(config);
    if let Err(err) = gemini_client.generate_image(&prompt).await {
        panic!("{}", err);
    };

    let ollama_client = OllamaClient::default();
    let res = match ollama_client.send_question(OllamaModel::Gemma3, &prompt).await {
        Ok(res) => res,
        Err(err) => panic!("Ollama error: {}", err),
    };

    println!("Question: {}\nResponse by {}\n\n{}", prompt, OllamaModel::Gemma3, res.response);
}
