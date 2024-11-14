/* FILENAME: llm/mod.rs
 * 
 * DESCRIPTION 
 * LLM Module, A Singleton providing a single access point for all queries to the Ollama LLM.
 * Generating context driven responses from the NPC.
 * 
 * 
 * NOTES
 * 
 * AUTHOR:    Rezwan Rahman  (RAH22529097)
 * CREATED:   04/11/2024
 * MODIFIED:  14/11/2024
 * 
 */

use ollama_rs::{generation::completion::{request::GenerationRequest, GenerationResponse}, Ollama};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use crate::error::{Error, Result};
mod system_prompts;

pub enum Models {
  Dolphin8B,
  Mistral7B,
  Gemma9B,
}

pub struct LLM {
  ollama: Ollama,
  model: String,
}

pub static LLM_INSTANCE: Lazy<Arc<LLM>> = Lazy::new(|| {
  Arc::new(LLM::new(Models::Mistral7B))
});

impl LLM {
  pub fn get_instance() -> Arc<LLM> {
    Arc::clone(&LLM_INSTANCE)
  }

  fn new(model: Models) -> Self {
    let model_name = match model {
        Models::Dolphin8B => "dolphin-llama3",
        Models::Mistral7B => "mistral",
        Models::Gemma9B => "gemma2:9b",
    };

    Self {
      ollama: Ollama::default(),
      model: model_name.to_string(),
    }
  }

  pub async fn generate_dialogue(&self, prompt: String) -> Result<GenerationResponse> {
    let response = self.ollama
      .generate(GenerationRequest::new( self.model.clone(), prompt)
      .system(system_prompts::DIALOGUE_SYSTEM_PROMPT.to_string()))
    .await?;

    Ok(response)
  }

  pub async fn generate_quest(&self) -> Result<GenerationResponse> {
    let response = self.ollama
      .generate(GenerationRequest::new( self.model.clone(), "Generate a Quest Suitable for the Player".to_string())
      .system(system_prompts::QUEST_SYSTEM_PROMPT.to_string()))
    .await?;

    Ok(response)
  }
}