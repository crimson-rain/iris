//! `agent/maestro.rs`

use crate::error::IrisGenError;
use ollama_rs::generation::chat::ChatMessage;
use super::model::Model;

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
        let resp = self.model.generate_request(&prompt, &mut self.history).await?;
        Ok(resp.message.content)
    }

    pub fn conduct_quest_gen(&self) -> String {
        "Conducted Quest Generation".to_string()
    }
}
