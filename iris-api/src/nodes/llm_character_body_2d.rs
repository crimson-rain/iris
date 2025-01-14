/* FILENAME: llm/mod.rs
 *
 * DESCRIPTION
 * Handles the LLM Logic and Instance.
 * Provides Multiple LLMs to Create.
 *
 *
 * NOTES
 * Make the Instance of the LLM a Singleton.
 *
 * AUTHOR:    Rezwan Rahman  (RAH22529097)
 * CREATED:   04/11/2024
 * MODIFIED:  12/11/2024
 *
 */

use std::{clone, sync::{Arc, Mutex}, thread};
use godot::{builtin::{Array, VariantArray}, global::{godot_error, godot_print}, meta::ToGodot, obj::{Base, WithBaseField}, prelude::{godot_api, GString, GodotClass, Variant}};
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use rig::providers::gemini::client::ApiResponse;
use tokio::{runtime::Runtime, sync::mpsc::{self, Receiver, Sender}};
use serde_json;
use crate::core::{dialogue_system, llm::{self, LLM}}; // Flatten This
use crate::core::dialogue_system::dialogue::Dialogue; // Flatten this to.
use crate::error::Error;
use std::time;


#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct LLMCharacterBody2D {
    // Character Exports and Information
    #[export]
    id: GString,
    #[export]
    profession: GString,
    #[export(multiline)]
    description: GString,

    // Base Class
    base: Base<CharacterBody2D>,

    // Temporary Memory Module for the NPC
    memory: Vec<String>,

    // Sender and Receiver for Transferring Data Between Threads.
    sender: Option<Sender<String>>,
    receiver: Option<Receiver<String>>,
}

#[godot_api]
impl ICharacterBody2D for LLMCharacterBody2D {
  fn init(base: Base<CharacterBody2D>) -> Self {
      let (sender, receiver) = mpsc::channel(1);

      Self {
          id: GString::from(""),
          profession: GString::from(""),
          description: GString::from(""),
          base,
          memory: Vec::new(),
          sender: Some(sender),
          receiver: Some(receiver),
      }
  }

  fn process(&mut self, _delta: f64) {
    // Collect all responses first to avoid holding the mutable borrow
    let mut responses = Vec::new();
    if let Some(receiver) = &mut self.receiver {
        while let Ok(response) = receiver.try_recv() {
            responses.push(response);
        }
    }

    // Now process each response
    for response in responses {
        godot_print!("{}", response);

        match serde_json::from_str::<Dialogue>(&response) {
            Ok(dialogue) => {
                let dialogue_text = GString::from(dialogue.dialogue);
                let mut choices_arr = VariantArray::new();

                for choices in dialogue.choices {
                    choices_arr.push(&GString::from(choices).to_variant());
                }

                self.base_mut().emit_signal(
                    "generated_dialogue",
                    &[Variant::from(dialogue_text), Variant::from(choices_arr)],
                );
            }
            Err(e) => {
                godot_print!("Failed to deserialize response to Dialogue: {:?}", e);
                self.base_mut().emit_signal(
                    "generated_dialogue",
                    &[Variant::from(GString::from("Error: Invalid response format"))],
                );
            }
        }
    }
  }
}

#[godot_api]
impl LLMCharacterBody2D {
  #[func]
  fn handle_interactions(&self, prompt: Variant) {
      if self.sender.is_none() || self.receiver.is_none() {
          godot_print!("Dialogue is already in Progress");
          return;
      }
  
      let llm_instance = LLM::get_instance();
      let user_prompt = match prompt.get_type() {
          godot::prelude::VariantType::NIL => None,
          godot::prelude::VariantType::STRING => Some(prompt.to_string()),
          _ => None,
      };
  
      let mut final_prompt = self.create_character_prompt();
      if let Some(ref extra) = user_prompt {
          final_prompt.push('\n');
          final_prompt.push_str(extra);
      }
  
      let prompt = self.create_character_prompt();
      let id = self.id.clone().to_string();
      let sender = self.sender.clone();
  
      thread::spawn(move || {
          let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
          runtime.block_on(async move {
              if let Ok(mut llm) = llm_instance.lock() {
                  if let Ok(response) = llm.generate_dialogue(prompt, id).await {
                      if let Some(sender) = sender {
                          let content = response.message.content.clone();
                          let _ = sender.send(content).await;
                      }
                  } else {
                      eprintln!("Failed to generate dialogue");
                  }
              } else {
                  eprintln!("Failed to lock the LLM instance");
              }
          });
      });
  }


  fn create_character_prompt(&self) -> String {
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
  fn generated_dialogue(&self, response: GString, choices: VariantArray);
}