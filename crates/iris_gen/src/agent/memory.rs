//! `agent/memory.rs`
//! 
//! Memory module is responsible for handling and managing memories for NPCs.
//! The memories are handled inside a history, which handles ChatMessages amongst the user and NPC.

use ollama_rs::generation::chat::ChatMessage;

pub struct Memory {
    history: Vec<ChatMessage>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            history: Vec::new(),
        }
    }
}

impl Memory {
    pub fn insert_memory(&mut self, message: ChatMessage) {
        if self.history.len() >= 30 {
            let summary = self.summarize_memory();
            self.history.clear();
            self.history.push(summary);
        }

        self.history.push(message);
    }

    pub fn get_memory(&self) -> Vec<ChatMessage> {
        self.history.clone()
    }

    fn summarize_memory(&self) -> ChatMessage {
        let summary = "Temporary Summary";

        ChatMessage {
            role: ollama_rs::generation::chat::MessageRole::System,
            content: summary.to_string(),
            tool_calls: vec![],
            images: None,
        }
    }
}
