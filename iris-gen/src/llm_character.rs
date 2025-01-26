//! This module is where the NPC is created.
//! Using Godot Binding we create a new NPC.
#![deny(clippy::todo)]

use crate::llm::LLM;
use crate::memory::MemoryStore;
use godot::builtin::GString;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::global::godot_print;
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};
use ollama_rs::generation::chat::ChatMessage;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct LLMCharacter {
    #[export]
    id: GString,
    #[export]
    profession: GString,
    #[export(multiline)]
    description: GString,

    base: Base<CharacterBody2D>,
    memory_store: MemoryStore,
    history: Vec<ChatMessage>,
    generation_channels: GenerationChannels,
}

struct GenerationChannels {
    dialogue_sender: Option<Sender<String>>,
    dialogue_reciever: Option<Receiver<String>>,
}

impl Default for GenerationChannels {
    fn default() -> Self {
        let (sender, reciver) = mpsc::channel(3);

        GenerationChannels {
            dialogue_sender: Some(sender),
            dialogue_reciever: Some(reciver),
        }
    }
}

#[godot_api]
impl ICharacterBody2D for LLMCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        LLMCharacter {
            id: GString::new(),
            profession: GString::new(),
            description: GString::new(),
            base,
            memory_store: MemoryStore::default(),
            history: Vec::new(),
            generation_channels: GenerationChannels::default(),
        }
    }

    fn process(&mut self, _delta: f64) {
        self.process_generated_dialogue();
    }
}

#[godot_api]
impl LLMCharacter {
    fn process_generated_dialogue(&mut self) {
        if let Some(receiver) = &mut self.generation_channels.dialogue_reciever {
            while let Ok(response) = receiver.try_recv() {
                godot_print!("CAPTURED: {}", response)
            }
        }
    }

    #[func]
    fn request_dialogue_generation(&mut self) {
        let mut llm = LLM::default();

        let prompt = "How is your day?";
        let npc_info = self.get_npc_info();
        let final_prompt = format!("Character Info: {}, Prompt: {}", npc_info, prompt);

        let mut history = self.history.clone();

        let sender = self.generation_channels.dialogue_sender.clone();
        let mut retrieve_memory = self.memory_store.retrieve_recent(3).clone();

        let runtime = Runtime::new().expect("Failed to Create Runtime");

        runtime.block_on(async move {
            if let Ok(response) = llm
                .generate_dialogue(final_prompt.as_str(), &mut history, &mut retrieve_memory)
                .await
            {
                if let Some(sender) = sender {
                    let _ = sender.send(response.message.content).await;
                }
            }
        });
    }

    fn get_npc_info(&self) -> String {
        format!(
            "You are role-playing as the following NPC:\n\
              Name: {}\n\
              Profession: {}\n\
              Description: {}\n\
              Please respond in character to the player's inquiries.",
            self.id, self.profession, self.description
        )
    }

    #[signal]
    fn dialogue_generated(&self, response: GString);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_response_channels() {
        let mut channels = GenerationChannels::default();

        if let Some(sender) = &channels.dialogue_sender {
            sender.send("Test Message".to_string()).await.unwrap();
        }

        if let Some(receiver) = &mut channels.dialogue_reciever {
            assert_eq!(receiver.try_recv().unwrap(), "Test Message".to_string());
        }
    }
}
