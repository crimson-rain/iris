//! This module provides functionality for creating and managing NPC memories.

use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct LLMCharacter;

#[godot_api]
impl ICharacterBody2D for LLMCharacter {
    fn init(_base: Base<CharacterBody2D>) -> Self {
        LLMCharacter
    }

    fn process(&mut self, _delta: f64) {}
}

// Test for the Library
#[godot_api]
impl LLMCharacter {}

#[cfg(test)]
mod tests {}
