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
use crate::memory::Memory;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::{ChatMessage, ChatMessageResponse};
use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;
use ollama_rs::generation::embeddings::GenerateEmbeddingsResponse;
use ollama_rs::Ollama;

pub const DOCUMENTS_PATH: &str = "";
pub const TOKENIZER_MODEL: &str = "bert-base-cased";
pub const MAX_TOKENS: usize = 1000;
pub const MODEL: &str = "phi4";

pub const DIALOGUE_SYSTEM: &'static str = r#"
  You are an NPC in a role-playing game. Use the provided character information to generate responses that are authentic to the character's persona.

  Format your response as:
  {
    "dialogue": "Your dialogue here.",
    "npc": "The NPC's name here.",
    "choices": ["Choice 1", "Choice 2", "Chocie 3"]
  }

  IMPORTANT: Do not include additional text or comments. Do NOT ADD JSON Formatting.
"#;

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
            model: model.to_string(),
        }
    }

    // TODO: Comment
    pub async fn generate_dialogue(
        &mut self,
        prompt: &str,
        history: &mut Vec<ChatMessage>,
        memory: &mut Vec<&Memory>,
    ) -> Result<ChatMessageResponse, Error> {
        let memory_string = memory
            .iter()
            .map(|m| m.memory_to_str())
            .collect::<Vec<String>>()
            .join(", ");

        let dialogue_request = ChatMessageRequest::new(
            self.model.clone(),
            vec![ChatMessage::user(format!(
                "SYSTEM: {}, PROMPT: {} MEMORY: {}",
                DIALOGUE_SYSTEM.to_string(),
                prompt.to_string(),
                memory_string,
            ))],
        );

        let res = self
            .ollama
            .send_chat_messages_with_history(history, dialogue_request)
            .await?;

        Ok(res)
    }

    pub async fn generate_embeddings(
        &self,
        text: &str,
    ) -> Result<GenerateEmbeddingsResponse, Error> {
        let request = GenerateEmbeddingsRequest::new(self.model.clone(), text.into());
        let res = self.ollama.generate_embeddings(request).await?;

        Ok(res)
    }
}

// Test for the Library
#[cfg(test)]
mod tests {
    use crate::memory::MemoryStore;

    use super::*;
    use ollama_rs::generation::chat::MessageRole;

    #[tokio::test]
    async fn test_generate_dialogue() {
        let mut llm = LLM::default();
        
        let mut hist = Vec::new();
        hist.push(ChatMessage::new(
            MessageRole::User,
            "We are talking about games".to_string(),
        ));

        let mut memory = MemoryStore::new();
        memory.add_memory("You are a mighty warrior named Chicken".to_string());
        memory.add_memory("You live in Aetheria".to_string());
        memory.add_memory("You are a Knight in Townsville".to_string());


        let res = llm
            .generate_dialogue(
                "Hello, I'm looking to do an adventure",
                &mut hist,
                &mut memory.retrieve_recent(3),
            )
            .await;

        println!("{:?}", res.as_ref().unwrap().message.content);

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn test_generate_embeddings() {
        let llm = LLM::default();
        let res = llm.generate_embeddings("What colour is the sky?").await;
        assert!(res.is_ok())
    }
}
