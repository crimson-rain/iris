//! `agent/maestro.rs`
//! Maestro is responsible for making LLM calls as well as handling various other
//! tasks, is required to create the needed operation.

use super::model::Model;
use crate::error::IrisGenError;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs_macros::tool_group;

pub struct Maestro {
    model: Model,
    history: Vec<ChatMessage>,
}

impl Default for Maestro {
    fn default() -> Self {
        Self {
            model: Model::default(),
            history: Vec::new(),
        }
    }
}

impl Maestro {
    pub async fn conduct_dialogue_gen(&mut self, prompt: String) -> Result<String, IrisGenError> {
        let tools = tool_group![super::tools::get_weather, super::tools::get_cpu_temperature];
        let resp = self
            .model
            .generate_request_with_tools(&prompt, self.history.clone(), tools)
            .await?;
        Ok(resp.message.content)
    }

    pub fn conduct_quest_gen(&self) -> String {
        "Conducted Quest Generation".to_string()
    }
}
