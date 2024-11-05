/* LLM
 * 
 * Purpose:
 * 
 * 
 * 
 * Author: Rezwan Rahman (RAH22529097)
 * Date Modified: 04/11/2024
*/

use ollama_rs::Ollama;

pub struct LLM {
  pub ollama: Ollama,
  pub model: Models,
}

// Models Enums, Stores Different Models used by the Quest and Dialogue Generation
pub enum Models {
  Dolphin,
  Mixtral,
  Gemma9B,
  Mistral7B,
}

impl Models {
  // Convert Enum to String Slice
  pub fn get_model(&self) -> String {
    match self {
      Models::Dolphin => "dolphin-llama3".to_string(),
      Models::Mixtral => "mixtral".to_string(),
      Models::Gemma9B => "gemma2:9b".to_string(),
      Models::Mistral7B => "mistral".to_string(),
    }
  }

  pub fn initialize_model(&self) -> LLM {
    match self {
      Models::Dolphin => {
        LLM {
          ollama: Ollama::default(),
          model: Models::Dolphin,
        }
      }
      Models::Mixtral => {
        LLM {
          ollama: Ollama::default(),
          model: Models::Mixtral,
        }
      }
      Models::Gemma9B => {
        LLM {
          ollama: Ollama::default(),
          model: Models::Gemma9B,
        }
      }
      Models::Mistral7B => {
        LLM {
          ollama: Ollama::default(),
          model: Models::Mistral7B,
        }
      }
    }
  }
}

pub enum SystemPrompt {
  DialogueSystem,
  QuestSystem,
}

impl SystemPrompt {
  // Convert Enum to String Slice
  pub fn get_system_prompt(&self) -> String {
    match self {
      SystemPrompt::DialogueSystem => {
        r"
        You are to act as a Non-Player Character Inside a Game.
        You are given information about the character such as their name, profession and general details about the character as well as their relationships with other characters.
        You are also given a general description of the characters traits etc.
        You must only talk to the character and nothing more.

        When the Player Interacts with you you are to act as the prompt given.
        
        Do not mention that you are a large language model.
        Only respond to approriate questions depenedent on the game world.
        Act like a Non-player Character.

        Make the dialogue short and concise.

        If the same question is asked only respond with the amount of times the question was asked.
        ".to_string()
      },

      SystemPrompt::QuestSystem => {
        r"
        You are to act as a Non-Player Character Inside a Game.
        Generate the quest for the player based on the information provided.
        ".to_string()
      },
    }
  }
}