/* FILENAME: llm_character.rs
 *
 * DESCRIPTION
 *
 * Responsible for LLM calls and natural language processing.
 * Generating dialogues and quest using system prompt.
 *
 * NOTES
 *
 * AUTHOR:    Rezwan Rahman (RAH22529097)
 * CREATED:   16/11/2024
 * MODIFIED:  16/11/2024
*/

use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct LLMCharacter;

#[godot_api]
impl ICharacterBody2D for LLMCharacter {
    fn init(_base: Base<CharacterBody2D>) -> Self {
        todo!()
    }

    fn process(&mut self, _delta: f64) {
        todo!()
    }
}

// Test for the Library
#[godot_api]
impl LLMCharacter {}

#[cfg(test)]
mod tests {}
