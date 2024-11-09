/* FILENAME: llm/mod.rs
 *
 * DESCRIPTION
 * Handles the LLM Logic and Instance.
 * Provides Multiple LLMs to Create.
 *
 *
 * NOTES
 * Keep this file simple and abstract everything to another layer.
 *
 * AUTHOR:    Rezwan Rahman  (RAH22529097)
 * CREATED:   04/11/2024
 * MODIFIED:  06/11/2024
 *
 */

mod system_prompts;

use ollama_rs::{generation::completion::{request::GenerationRequest, GenerationResponse}, Ollama};
use crate::prelude::*;
use system_prompts::SystemPrompt;

pub enum Models {
  Dolphin,
  Mixtral,
  Gemma9B,
  Mistral7B
}

pub struct LLM {
  ollama: Ollama,
  model: String,
}

impl LLM {
  pub fn new(model: Models) -> Self {
    let model_name = match model {
      Models::Dolphin => "dolphin-llama3",
      Models::Mixtral => "mixtral",
      Models::Gemma9B => "gemma2:9b",
      Models::Mistral7B => "mistral",
    };

    LLM {
      ollama: Ollama::default(),
      model: model_name.to_string(),
    }
  }
  
  pub async fn generate_dialogue(&self, prompt: String) -> Result<GenerationResponse> {
    let response = self.ollama
      .generate(GenerationRequest::new( self.model.clone(), prompt)
      .system(SystemPrompt::DialogueSystem.get_system().to_string())
    )
      .await?;
    Ok(response)
  }

  pub async fn generate_quest(&self) -> Result<GenerationResponse> {
    let response = self.ollama
      .generate(GenerationRequest::new( self.model.clone(), "Generate a Quest Suitable for the Player".to_string())
      .system(SystemPrompt::QuestSystem.get_system().to_string())
    )
      .await?;
    Ok(response)
  }
}