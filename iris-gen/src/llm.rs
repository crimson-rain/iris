/* FILENAME: llm.rs
 *
 * DESCRIPTION
 *
 * Responsible for LLM calls and natural language processing.
 * Generating dialogues and quest using system prompt.
 *
 * NOTES / TODOS / FIXME
 * TODO: Create Test Cases
 *
 * AUTHOR:    Rezwan Rahman (RAH22529097)
 * CREATED:   16/11/2024
 * MODIFIED:  16/11/2024
*/

use crate::error::Error;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::{ChatMessage, ChatMessageResponse};
use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;
use ollama_rs::generation::embeddings::GenerateEmbeddingsResponse;
use ollama_rs::Ollama;

pub const DOCUMENTS_PATH: &str = "";
pub const TOKENIZER_MODEL: &str = "bert-base-cased";
pub const MAX_TOKENS: usize = 1000;
pub const MODEL: &str = "mistral";
pub const DIALOGUE_SYSTEM: &str = r"";
pub const QUEST_SYSTEM: &str = r"";

pub struct LLM {
    ollama: Ollama,
    model: String,
}

impl Default for LLM {
    fn default() -> Self {
        LLM {
            ollama: Ollama::default(),
            model: MODEL.to_string(),
        }
    }
}

impl LLM {
    // TODO: Comment
    pub fn new(model: &str) -> Self {
        LLM {
            ollama: Ollama::default(),
            model: model.to_string()
        }
    }

    // TODO: Comment
    pub async fn generate_dialogue(
        &mut self,
        prompt: &str,
        history: &mut Vec<ChatMessage>,
    ) -> Result<ChatMessageResponse, Error> {
        let dialogue_request = ChatMessageRequest::new(
            self.model.clone(),
            vec![ChatMessage::user(format!(
                "SYSTEM: {}, PROMPT: {}",
                DIALOGUE_SYSTEM.to_string(),
                prompt.to_string()
            ))],
        );

        let res = self
            .ollama
            .send_chat_messages_with_history(history, dialogue_request)
            .await?;

        Ok(res)
    }

    // TODO: Comment
    pub async fn generate_quest(
        &mut self,
        prompt: &str,
        history: &mut Vec<ChatMessage>,
    ) -> Result<ChatMessageResponse, Error> {
        let quest_request = ChatMessageRequest::new(
            self.model.clone(),
            vec![ChatMessage::user(format!(
                "SYSTEM: {}, PROMPT: {}",
                QUEST_SYSTEM.to_string(),
                prompt.to_string()
            ))],
        );

        let res = self
            .ollama
            .send_chat_messages_with_history(history, quest_request)
            .await?;

        Ok(res)
    }

    pub async fn generate_embeddings(&self, text: &str) -> Result<GenerateEmbeddingsResponse, Error> {
        let request = GenerateEmbeddingsRequest::new(self.model.clone(), text.into());
        let res = self.ollama.generate_embeddings(request).await?;
        
        Ok(res)
    }
}

// Test for the Library
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_embeddings() {
        let llm = LLM::default();
        let res = llm.generate_embeddings("What colour is the sky?").await;

        assert!(res.is_ok())
    }
}
