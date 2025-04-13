//! `agent/maestro.rs`
//! Maestro is responsible for making LLM calls as well as handling various other
//! tasks, is required to create the needed operation.

use super::model::Model;
use crate::error::IrisGenError;
use ollama_rs::generation::chat::ChatMessage;

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
        let resp = self
            .model
            .generate_request_with_tools(&prompt, self.history.clone())
            .await?;

        self.history.push(ChatMessage::new(ollama_rs::generation::chat::MessageRole::User, prompt));
        self.history.push(ChatMessage::new(ollama_rs::generation::chat::MessageRole::Assistant, resp.message.content.clone()));

        Ok(resp.message.content)
    }

    pub async fn conduct_quest_gen(&self) -> Result<String, IrisGenError> {
        Ok("Conducted Quest Generation".to_string())
    }

    pub async fn conduct_embed_gen(&self, data: String) -> Result<Vec<Vec<f32>>, IrisGenError> {
        let embeds = self.model.generate_embeddings(&data).await.unwrap();
        Ok(embeds.embeddings)
    }

    pub async fn conduct_rag(&self, prompt: String) -> Result<String, IrisGenError> {
        todo!()
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
                .conduct_dialogue_gen("How are you?, Mel".to_string())
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
