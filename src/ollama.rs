use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use anyhow::Result;

#[derive(Debug)]
pub enum OllamaModel {
    Gemma3,
}
pub struct OllamaClient {
    ollama: Ollama,
    instruction: String,
}

impl std::fmt::Display for OllamaModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let model = match self {
            OllamaModel::Gemma3 => "gemma3:4b",
        };

        write!(f, "{}", model)
    }
}

impl OllamaClient {
    pub fn new(host: &str, port: u16, instruction: String) -> Self {
        Self {
            ollama: Ollama::new(host, port),
            instruction: instruction,
        }
    }

    pub async fn send_question(
        &self,
        model: OllamaModel,
        prompt: &str,
    ) -> Result<String> {
        let inst_prompt = format!("{} {}", self.instruction, prompt);

        let resp = self
            .ollama
            .generate(GenerationRequest::new(model.to_string(), inst_prompt))
            .await?;
                
        let trimmed_json_block = Self::trim_ollama_resp(resp.response);

        Ok(trimmed_json_block)
    }

    pub fn trim_ollama_resp(resp_str: String) -> String {
        resp_str
            .chars()
            .skip(7)
            .take(resp_str.chars().count() - 7 - 3)
            .collect()
    }
}
