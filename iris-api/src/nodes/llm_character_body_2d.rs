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

use std::{clone, sync::Arc, thread};
use godot::{global::{godot_error, godot_print}, obj::{Base, WithBaseField}, prelude::{godot_api, GString, GodotClass, Variant}};
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use tokio::{runtime::Runtime, sync::mpsc::{self, Receiver, Sender}};
use serde_json;
use crate::core::{dialogue, llm::{self, LLM, LLM_INSTANCE}}; // Flatten This
use crate::core::dialogue::Dialogue; // Flatten this to.



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
      if let Some(receiver) = &mut self.receiver {
        if let Ok(response) = receiver.try_recv() {
          // FIXME: Handle the error where String cannot be converted to Dialogue Struct
          match serde_json::from_str::<Dialogue>(&response) {
            Ok(dialogue) => {
              self.base_mut().emit_signal("generated_dialogue", &[Variant::from(GString::from(dialogue.dialogue))]);
            }
            Err(e) => {
              godot_print!("Received JSON response {:?}", response);
              godot_print!("Failed to Parse Dialogue: {:?}", e);
              self.handle_interactions();
            }
          }
        }
      }
    }
}

#[godot_api]
impl LLMCharacterBody2D {
  #[func]
  fn handle_interactions(&self) {
    let llm = LLM::get_instance();

    let sender = self.sender.clone();
    let prompt = self.create_character_prompt();

    thread::spawn(move || {
      let runtime = Runtime::new().expect("Failed to Create Tokio Runtime");

      runtime.block_on(async move {
        if let Ok(response) = llm.generate_dialogue(prompt).await {
          if let Some(sender) = sender {
            let _ = sender.send(response.response).await;
          }
        }
      });
    });
  }

  fn create_character_prompt(&self) -> String {
    format!(
        "Character Name: {}\nProfession: {}\nDescription: {}",
        self.id, self.profession, self.description
    )
  }

  #[signal]
  fn generated_dialogue(&self, response: GString);
}