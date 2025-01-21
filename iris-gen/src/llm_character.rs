//! This module is where the NPC is created.
//! Using Godot Binding we create a new NPC.
#![deny(clippy::todo)]

use godot::builtin::GString;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};

use crate::memory::MemoryStore;

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
    memory_store: MemoryStore
}

#[godot_api]
impl ICharacterBody2D for LLMCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        LLMCharacter {
            id: GString::new(),
            profession: GString::new(),
            description: GString::new(),
            base,
            memory_store: MemoryStore::default()
        }
    }

    fn process(&mut self, _delta: f64) {}
}

#[godot_api]
impl LLMCharacter {}

#[cfg(test)]
mod tests {}
