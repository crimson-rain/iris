/* FILENAME: llm/mod.rs
 * 
 * DESCRIPTION 
 * LLM Module, a singleton providing a single access point for all queries to the Ollama LLM.
 * Generating context-driven responses from the NPC.
 * 
 * NOTES
 * 
 * AUTHOR:    Rezwan Rahman (RAH22529097)
 * CREATED:   04/11/2024
 * MODIFIED:  14/11/2024
 */

use godot::global::godot_print;
use ollama_rs::{generation::{chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponse}, completion::{request::GenerationRequest, GenerationResponse}}, Ollama};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use crate::error::{Error, Result};
mod system_prompts;

pub enum ModelType {
  Dolphin8B,
  Mistral7B,
  Gemma9B,
}

impl ModelType {
  fn to_model_name(&self) -> &'static str {
    match self {
      ModelType::Dolphin8B => "dolphin-llama3",
      ModelType::Mistral7B => "mistral",
      ModelType::Gemma9B => "gemma2:9b",
    }
  }
}

pub struct LLM {
  ollama: Mutex<Ollama>,
  model: String,
}

pub static LLM_INSTANCE: Lazy<Arc<LLM>> = Lazy::new(|| {
  Arc::new(LLM::new(ModelType::Dolphin8B))
});

impl LLM {
  /// Get the singleton instance of the LLM.
  pub fn get_instance() -> Arc<LLM> {
    Arc::clone(&LLM_INSTANCE)
  }

  /// Create a new LLM instance with the specified model.
  fn new(model: ModelType) -> Self {
    let model_name = model.to_model_name();

    Self {
      ollama: Mutex::new(Ollama::default()),
      model: model_name.to_string(),
    }
  }

  /// Generate dialogue based on a given prompt.
  pub async fn generate_dialogue(&self, prompt: String, id: String) -> Result<ChatMessageResponse> {
    // Lock the Mutex to get mutable access to Ollama
    let mut ollama = self.ollama.lock().expect("Failed to lock Mutex");

    // Proceed with generating dialogue
    let response = ollama
        .send_chat_messages_with_history(
            ChatMessageRequest::new(
                self.model.clone(),
                vec![
                  ChatMessage::system(format!("{}, {}", system_prompts::DIALOGUE_SYSTEM_PROMPT.to_string(), prompt)),
                ],
            ),
            id,
        )
        .await?;      
    Ok(response)
  }

  /// Generate a quest for the player.
  pub async fn generate_quest(&self) -> Result<GenerationResponse> {
    let mut ollama = self.ollama.lock().expect("Failed to lock Mutex");
    
    let response = ollama
      .generate(GenerationRequest::new(self.model.clone(), "Generate a Quest Suitable for the Player".to_string())
        .system(system_prompts::QUEST_SYSTEM_PROMPT.to_string()))
      .await?;

    Ok(response)
  }
}
 