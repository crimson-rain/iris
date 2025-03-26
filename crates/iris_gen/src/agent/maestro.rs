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

    pub async fn conduct_quest_gen(&self) -> Result<String, IrisGenError> {
        Ok("Conducted Quest Generation".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_conduct_dialogue_gen() {
        let mut maestro = Maestro::default();

        assert!(
            maestro
                .conduct_dialogue_gen("What is the Weather in Tokyo?".to_string())
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_conduct_quest_gen() {
        let maestro = Maestro::default();
        assert!(maestro.conduct_quest_gen().await.is_ok())
    }
}
