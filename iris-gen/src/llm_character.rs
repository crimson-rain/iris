//! This module is where the NPC is created. 
//! Using Godot Binding we create a new NPC. 
#![deny(clippy::todo)]

use godot::builtin::GString;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct LLMCharacter {
    base: Base<CharacterBody2D>,
    #[export]
    id: GString,
    #[export]
    profession: GString,
    #[export(multiline)]
    description: GString,
}

#[godot_api]
impl ICharacterBody2D for LLMCharacter {
    fn init(base: Base<CharacterBody2D>) -> Self {
        LLMCharacter {
            base,
            id: GString::new(),
            profession: GString::new(),
            description: GString::new(),
        }
    }

    fn process(&mut self, _delta: f64) {}
}

#[godot_api]
impl LLMCharacter {}

#[cfg(test)]
mod tests {}
