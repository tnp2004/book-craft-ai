use ollama_rs::{
    Ollama,
    error::OllamaError,
    generation::completion::{GenerationResponse, request::GenerationRequest},
};

use crate::file::{File, Instruction};

#[derive(Debug)]
pub enum OllamaModel {
    Gemma3,
}
pub struct OllamaClient {
    ollama: Ollama,
    instruction: Instruction
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
            instruction: File::read_instruction("instruction.json"),
        }
    }
}

impl OllamaClient {
    pub fn new(host: String, port: u16, instruction: Instruction) -> Self {
        Self {
            ollama: Ollama::new(host, port),
            instruction: instruction,
        }
    }

    pub async fn send_question(
        &self,
        model: OllamaModel,
        prompt: &str,
    ) -> Result<GenerationResponse, OllamaError> {
        let inst_prompt = format!("{}\n{}", self.instruction.instruction, prompt);
        
        let res = self
            .ollama
            .generate(GenerationRequest::new(model.to_string(), inst_prompt))
            .await;

        return res;
    }
}
