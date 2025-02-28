//! This module provides functionality to interact with the LLM using Ollama's API.
//! Defines the `Model` struct and methods which are associated with it.

use crate::error::IrisError;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::{ChatMessage, ChatMessageResponse};
use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;
use ollama_rs::generation::embeddings::GenerateEmbeddingsResponse;
use ollama_rs::Ollama;

pub const MODEL: &str = "phi4";

pub const DIALOGUE_SYSTEM: &str = r#"
  You are an NPC in a role-playing game. Stay in character at all times and respond in a manner authentic to the NPC's persona.
  
  ## NPC Details:
  - **Name**: {npc_name}
  - **Role**: {npc_role} (e.g., Merchant, Guard, Mage)
  - **Personality**: {npc_personality} (e.g., Grumpy, Helpful, Mysterious)
  - **Backstory**: {npc_backstory}
  - **Relationship to Player**: {npc_relationship}

  ## Response Guidelines:
  - **Be concise**: Avoid long responses unless necessary.
  - **Be immersive**: Your responses should match the world, lore, and NPC's knowledge.
  - **Follow constraints**: Never break the fourth wall or acknowledge being an AI.
  - **Context-aware**: Adjust responses based on previous dialogue and player choices.

  ## Memory Awareness:
  - Use the following **memories** to inform your response: {npc_memory}.
  - If memory is relevant, naturally reference past events **without forcing it**.
  - If no relevant memory applies, do not mention past events.

  ## Choice Guidelines:
  - Each choice should be **meaningful** and relevant to the conversation.
  - Offer **different paths** (e.g., action-based, inquiry, or neutral response).
  - Avoid duplicate or redundant choices.
  - At least one choice should lead to dialogue progression.

  ## Strict JSON Structure:
  - Return output **ONLY** in valid JSON format (do not include explanations).
  - Do **not** add escape characters, markdown, or formatting hints.
  - Do **not** include extra commentary (e.g., "Here's your JSON: ...").

  ## Response Format (DO NOT ALTER):
  {
    "dialogue": "NPC's response here.",
    "npc": "{npc_name}",
    "choices": ["Option 1", "Option 2", "Option 3"]
  }
"#;

pub struct Model {
    ollama: Ollama,
    model: String,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            ollama: Ollama::default(),
            model: MODEL.to_string(),
        }
    }
}

impl Model {
    pub async fn generation_request(
        &mut self,
        prompt: &str,
        history: &mut Vec<ChatMessage>,
    ) -> Result<ChatMessageResponse, IrisError> {
        if history.is_empty() {
            history.push(ChatMessage::system(DIALOGUE_SYSTEM.to_string()));
        }

        // Create Request for Ollama API
        let req = ChatMessageRequest::new(
            self.model.clone(),
            vec![ChatMessage::user(prompt.to_string())],
        );

        // Handle Response Generated
        let res = self
            .ollama
            .send_chat_messages_with_history(history, req)
            .await?;

        // Return Result if Success
        Ok(res)
    }

    pub async fn generate_embeddings(
        &self,
        raw_text: &str,
    ) -> Result<GenerateEmbeddingsResponse, IrisError> {
        // Embedding Request to Model
        let req = GenerateEmbeddingsRequest::new(self.model.clone(), raw_text.into());

        // Generated Embeddings
        let res = self.ollama.generate_embeddings(req).await?;

        Ok(res)
    }
}
