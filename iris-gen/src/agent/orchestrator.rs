use ollama_rs::generation::chat::ChatMessage;

use super::model::Model;
use crate::error::IrisError;
use crate::memory::Memory;
use crate::schemas::dialogue::Dialogue;

#[derive(Default)]
pub struct Orchestrator {
    model: Model,
}

impl Orchestrator {
    pub async fn orchestrate_dialogue(
        &mut self,
        history: &mut Vec<ChatMessage>,
        memory: &mut Vec<Memory>,
        prompt: &str,
    ) -> Result<Dialogue, IrisError> {
        
        // Iterate Memories and Convert to String
        let memories = memory
            .iter()
            .map(|mem| mem.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let combined_prompt = format!("Memory: {},  Prompt: {}", memories, prompt);

        // Generateion Request to the Model
        let res = self
            .model
            .generation_request(&combined_prompt, history)
            .await?;

        // Parse Generated Text to Dialogue
        let dialogue = Dialogue::try_from(res.message.content.as_str())?;

        // Return Dialogue
        Ok(dialogue)
    }

    pub async fn orchestrate_quest(&self) {
        unimplemented!("Not Implemented Currently")
    }
}
