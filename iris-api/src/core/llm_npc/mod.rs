/* FILENAME: llm/mod.rs
 * 
 * DESCRIPTION 
 * This creates a class/node for godot to interact with the LLM.
 * We can use the node as scaffolding for the LLM.
 * 
 * 
 * NOTES
 * Keep this file simple and abstract everything to another layer.
 * 
 * AUTHOR:    Rezwan Rahman  (RAH22529097)
 * CREATED:   04/11/2024
 * MODIFIED:  06/11/2024
 * 
 */

use std::sync::Arc;

use godot::{builtin::GString, classes::{CharacterBody2D, ICharacterBody2D}, global::godot_print, obj::Base, prelude::{godot_api, GodotClass}};
use tokio::runtime::Runtime;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct LLMCharacterBody2D {
  #[export]
  id: GString,
  memory: Vec<String>,
  base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for LLMCharacterBody2D {
  fn init(base: Base<CharacterBody2D>) -> Self {
    Self {
      id: GString::from(""),
      memory: vec![],
      base,
    }
  }

  fn ready(&mut self) {
    godot_print!("Successfully Created LLMNPC");
  }
}