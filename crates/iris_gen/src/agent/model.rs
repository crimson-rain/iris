//! `agent/model.rs`

use ollama_rs::Ollama;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::{ChatMessage, ChatMessageResponse};
use ollama_rs::generation::embeddings::GenerateEmbeddingsResponse;
use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;

use crate::error::IrisGenError;

use super::configs::{EMBED_MODEL, LLM_MODEL};

#[derive(Clone)]
pub struct Model {
    ollama: Ollama,
    llm_model: String,
    embed_model: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            ollama: Ollama::default(),
            llm_model: LLM_MODEL.to_string(),
            embed_model: EMBED_MODEL.to_string(),
        }
    }
}

impl Model {
    pub async fn generate_request(
        &mut self,
        prompt: &str,
        history: &mut Vec<ChatMessage>,
    ) -> Result<ChatMessageResponse, IrisGenError> {
        let req = ChatMessageRequest::new(
            self.llm_model.clone(),
            vec![ChatMessage::user(prompt.to_string())],
        );

        let res = self
            .ollama
            .send_chat_messages_with_history(history, req)
            .await?;

        Ok(res)
    }

    pub async fn generate_embeddings(
        &self,
        raw_text: &str,
    ) -> Result<GenerateEmbeddingsResponse, IrisGenError> {
        let req = GenerateEmbeddingsRequest::new(self.embed_model.clone(), raw_text.into());

        let res = self.ollama.generate_embeddings(req).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::configs::DIALOGUE_SYSTEM_PROMPT;

    #[tokio::test]
    async fn test_generation_request() {
        let mut model = Model::default();
        let mut hist = Vec::new();

        hist.push(ChatMessage::new(
            ollama_rs::generation::chat::MessageRole::System,
            DIALOGUE_SYSTEM_PROMPT.to_string(),
        ));

        let prompt = "Hello, World!";

        let generated_text = model.generate_request(prompt, &mut hist).await;

        assert!(generated_text.is_ok(), "Failed to Generate Text");
        assert!(!hist.is_empty(), "History is Empty");
        assert_eq!(
            hist.len(),
            3,
            "Chat History wasn't Updated from Last Generation."
        );
    }

    #[tokio::test]
    async fn test_rag_generation_request() {
        let model = Model::default();

        let raw_text = "Hello, World!";

        let generated_embedding = model.generate_embeddings(raw_text).await;

        assert!(generated_embedding.is_ok(), "Failed to Generate Embeddings");
    }
}
