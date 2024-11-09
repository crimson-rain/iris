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
use ollama_rs::{generation::completion::{request::GenerationRequest, GenerationResponse}, Ollama};
use crate::prelude::*;
 
const DIALOGUE_SYSTEM_PROMPT: &'static str =
r"
  You are to act as a Non-Player Character Inside a Game.
  You are given information about the character such as their name, profession and general details about the character as well as their relationships with other characters.
  You are also given a general description of the characters traits etc.
  You must only talk to the character and nothing more.
  When the Player Interacts with you you are to act as the prompt given.
 
  Do not mention that you are a large language model.
  Only respond to approriate questions depenedent on the game world.
  Act like a Non-player Character.

  Ensure that you dont go over 300 words.
";

const QUEST_SYSTEM_PROMPT: &'static str =
r"
  You are to act as a Non-Player Character Inside a Game.
  Generate the quest for the player based on the information provided.
";

pub enum Models {
  Dolphin,
  Mixtral,
  Gemma9B,
  Mistral7B
}

pub enum SystemPrompt {
  DialogueSystem,
  QuestSystem,
}

impl SystemPrompt {
  pub fn get_system(&self) -> &'static str {
    match *self {
        SystemPrompt::DialogueSystem => DIALOGUE_SYSTEM_PROMPT,
        SystemPrompt::QuestSystem => QUEST_SYSTEM_PROMPT,
    }
  }
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