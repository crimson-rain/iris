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
            .await;

        res.map_err(Error::OllamaGenerationError)
    }

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
            .await;

        res.map_err(Error::OllamaGenerationError)
    }
}

// Test for the Library
#[cfg(test)]
mod tests {}
