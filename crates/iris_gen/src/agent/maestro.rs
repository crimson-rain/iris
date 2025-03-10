//! `agent/maestro.rs`

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
    pub async fn conduct_dialogue_gen(&mut self) -> String {
        self.model.generate_request("How is your day?", &mut self.history).await.unwrap().message.content
    }

    pub fn conduct_quest_gen(&self) -> String {
        "Conducted Quest Generation".to_string()
    }
}
