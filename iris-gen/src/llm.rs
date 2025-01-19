//! This module provides functionality for interacting with Ollama API.
//!
//! It defines the `LLM` struct and methods which are associated with it.
//! It is responsible for making API calls to the Ollama API.
//!
//! ### Features
//! - Create and Configure LLM to Interact To
//! - Generating Dialogue Responses for NPCs and Embedding for Vector Stores

use crate::error::IrisError;
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

pub const DIALOGUE_SYSTEM: &str = r#"
  You are an NPC in a role-playing game. Use the provided character information to generate responses that are authentic to the character's persona.

  Format your response as:
  {
    "dialogue": "Your dialogue here.",
    "npc": "The NPC's name here.",
    "choices": ["Choice 1", "Choice 2", "Chocie 3"]
  }

  IMPORTANT: Do not include additional text or comments. Do NOT ADD JSON Formatting.
"#;

/// Struct representing the LLMs used to Interact with Ollama API.
///
/// The `LLM` struct is the primary interface for making API calls to Ollama for generating
/// dialogue, embeddings, or other language tasks. It encapsulates the Ollama client and
/// The model name is being used for these tasks.
pub struct LLM {
    /// The Ollama client used for communicating with the Ollama API
    ollama: Ollama,

    /// The name of the model used for generating responses or embeddings.
    /// Examples: "phi4", "mistral", "llama3.2:latest"
    model: String,
}

/// Provides the default implementation for the `LLM` struct.
///
/// The default implementation initializes the `LLM` struct with:
/// - A default-configured `Ollama` client.
/// - A model name defined by the constant `MODEL`.
///
/// This lets users quickly initialise an `LLM` instance with sensible defaults.
///
/// # Example
/// ```
/// use iris_gen::llm::LLM;
/// 
/// let llm = LLM::default();
/// ```
impl Default for LLM {
    fn default() -> Self {
        LLM {
            ollama: Ollama::default(),
            model: MODEL.to_string(),
        }
    }
}

impl LLM {
    /// Represents a constructor for the `LLM` struct, which is used to interact
    /// with the Ollama API via the `ollama-rs` library.
    ///
    /// ### Arguments
    /// * `model`   - A string slice (`&str`) representing the name of the model
    ///             to be used (e.g., "mistral", "llama3.2:latest", "phi4").
    ///
    /// ### Returns
    /// An initialised `LLM` instance with the specified model name.
    ///
    /// ### Example
    /// ```
    /// use iris_gen::llm::LLM;
    /// 
    /// // Create a new instance of LLM with the "mistral" model.
    /// let llm = LLM::new("mistral");
    /// ```
    pub fn new(model: &str) -> Self {
        LLM {
            ollama: Ollama::default(),
            model: model.to_string(),
        }
    }

    /// Generate dialogue for an NPC based on the given, prompt, chat history, and memory.
    ///
    /// This method interacts with the Ollama API to produce dialogue which is relevant to the NPC's
    /// character, persona, and memory context. It uses the system instructions prompts, history, and memory to
    /// create contextually driven and authentic dialogue.
    ///
    /// ### Arguments
    /// * `prompt` - A string slice (`&str`) which holds the prompt or instructions
    ///              for dialogue generation.
    ///
    /// * `history` - A vector which is made up of `ChatMessage` struct, that holds
    ///              the message role (i.e user, tool, system..) and the contents of the message used to store previous messages.
    ///
    /// * `memory` - A vector of references to `Memory` structs representing the character's persona and background.
    ///               It should contain relevant facts to maintain dialogue consistency, avoiding excessive storage for performance reasons.
    ///
    /// ### Returns
    /// - `Ok(ChatMessageResponse)` - A successful response containing the generated NPC dialogue.
    /// - `Err(IrisError)` - An error if the API request fails or returns an invalid/malformed response.
    ///
    /// ### Example
    /// ```
    /// use iris_gen::llm::LLM;
    /// use ollama_rs::generation::chat::{ChatMessage, MessageRole};
    /// use iris_gen::memory::Memory;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut llm = LLM::default();
    ///     
    ///     let mut history = vec![
    ///         ChatMessage::new(MessageRole::User, "We are talking about adventures.".to_string()),
    ///     ];
    ///     
    ///     let mut memory = vec![
    ///         Memory::new(format!("You are a brave knight in Aetheria.")),
    ///         Memory::new(format!("You have a trusty steed named Thunder.")),
    ///     ];
    ///     
    ///     let prompt = "I need advice on my next quest.";
    ///     
    ///     let response = llm
    ///         .generate_dialogue(prompt, &mut history, &mut memory.iter().collect())
    ///         .await;
    ///     
    ///     match response {
    ///         Ok(res) => {
    ///             println!("Generated dialogue: {}", res.message.content);
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Error generating dialogue: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Notes
    /// - The `DIALOGUE_SYSTEM` constant defines the system prompt for generating NPC-specific responses.
    /// - The `history` vector is updated to include the generated response.
    /// - The `memory` vector helps maintain consistency in the NPC's persona and contextual knowledge.
    pub async fn generate_dialogue(
        &mut self,
        prompt: &str,
        history: &mut Vec<ChatMessage>,
        memory: &mut Vec<&Memory>,
    ) -> Result<ChatMessageResponse, IrisError> {
        let memory_string = memory
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let dialogue_request = ChatMessageRequest::new(
            self.model.clone(),
            vec![ChatMessage::user(format!(
                "SYSTEM: {}, PROMPT: {} MEMORY: {}",
                DIALOGUE_SYSTEM, prompt, memory_string,
            ))],
        );

        let res = self
            .ollama
            .send_chat_messages_with_history(history, dialogue_request)
            .await?;

        Ok(res)
    }

    /// Generate embeddings to store in a vector database. For later use such as RAG.
    /// TODO: Implement Vector Database to Project
    /// This method interacts with the Ollama API to produce an embedding it transforms
    /// text into vector values to store in the vector database.
    ///
    /// ### Arguments
    /// * `text` - A string slice (`&str`) which holds the text to embed.
    ///
    /// ### Returns
    /// - `Ok(GenerateEmbeddingsResponse)` - A successful response containing the generated embeddings.
    /// - `Err(IrisError)` - An error if the API request fails or returns an invalid/malformed response.
    ///
    /// ### Example
    /// ```
    /// use iris_gen::llm::LLM;
    /// 
    /// # #[tokio::main]
    /// async fn main() {
    ///     let mut llm = LLM::default();
    ///     
    ///     let text = "Today is Sunday, and it's a Bright Morning Day...";
    ///     
    ///     let response = llm
    ///         .generate_embeddings(text)
    ///         .await;
    ///     
    ///     match response {
    ///         Ok(res) => {
    ///             println!("Generated dialogue: {:?}", res.embeddings);
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Error generating dialogue: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Notes
    /// - The generated embedding is stored in a Matrix of float-32 values.
    pub async fn generate_embeddings(
        &self,
        text: &str,
    ) -> Result<GenerateEmbeddingsResponse, IrisError> {
        let request = GenerateEmbeddingsRequest::new(self.model.clone(), text.into());
        let res = self.ollama.generate_embeddings(request).await?;

        Ok(res)
    }
}

///
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
