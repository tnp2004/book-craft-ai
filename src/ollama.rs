use ollama_rs::{
    Ollama,
    error::OllamaError,
    generation::completion::{GenerationResponse, request::GenerationRequest},
};

#[derive(Debug)]
pub enum OllamaModel {
    Gemma3,
}
pub struct OllamaClient {
    ollama: Ollama,
}

impl std::fmt::Display for OllamaModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let model = match self {
            OllamaModel::Gemma3 => "gemma3:4b",
        };

        write!(f, "{}", model)
    }
}

impl Default for OllamaClient {
    fn default() -> Self {
        Self {
            ollama: Ollama::default(),
        }
    }
}

impl OllamaClient {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            ollama: Ollama::new(host, port),
        }
    }

    pub async fn send_question(
        &self,
        model: OllamaModel,
        prompt: &str,
    ) -> Result<GenerationResponse, OllamaError> {
        let res = self
            .ollama
            .generate(GenerationRequest::new(model.to_string(), prompt))
            .await;

        return res;
    }
}
