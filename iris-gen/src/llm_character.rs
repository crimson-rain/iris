//! This module is where the NPC is created.
//! Using Godot Binding we create a new NPC.
#![deny(clippy::todo)]

use crate::memory::MemoryStore;
use godot::builtin::GString;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};
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
    generation_channels: GenerationChannels,
}

struct GenerationChannels {
    dialogue_sender: Option<Sender<String>>,
    dialogue_reciver: Option<Receiver<String>>,
}

impl Default for GenerationChannels {
    fn default() -> Self {
        let (sender, reciver) = mpsc::channel(1);

        GenerationChannels {
            dialogue_sender: Some(sender),
            dialogue_reciver: Some(reciver),
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
            generation_channels: GenerationChannels::default(),
        }
    }

    fn process(&mut self, _delta: f64) {
        self.handle_generation();
    }
}

#[godot_api]
impl LLMCharacter {
    fn handle_generation(&mut self) {
        let mut responses = Vec::new();
        if let Some(reciver) = &mut self.generation_channels.dialogue_reciver {
            while let Ok(response) = reciver.try_recv() {
                responses.push(response);
            }
        }
    }
}

#[cfg(test)]
mod tests {}
