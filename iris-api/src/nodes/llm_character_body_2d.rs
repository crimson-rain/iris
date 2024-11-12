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

use std::{sync::Arc, thread};
use godot::{obj::{Base, WithBaseField}, prelude::{godot_api, GString, GodotClass, Variant}};
use godot::classes::CharacterBody2D;
use tokio::{runtime::Runtime, sync::mpsc::{self, Receiver, Sender}};
use crate::core::llm::{self, LLM, LLM_INSTANCE};
use godot::classes::ICharacterBody2D;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct LLMCharacterBody2D {
    // Character Exports
    #[export]
    id: GString,
    #[export]
    profession: GString,
    #[export(multiline)]
    description: GString,

    memory: Vec<String>,
    base: Base<CharacterBody2D>,
    llm: Arc<LLM>,
    sender: Option<Sender<String>>,
    receiver: Option<Receiver<String>>,
}

#[godot_api]
impl ICharacterBody2D for LLMCharacterBody2D {
    fn init(base: Base<CharacterBody2D>) -> Self {
        let llm = Arc::new(LLM::new(llm::Models::Mistral7B));
        let (sender, receiver) = mpsc::channel(1);

        Self {
            id: GString::from(""),
            profession: GString::from(""),
            description: GString::from(""),
            memory: Vec::new(),
            base,
            llm,
            sender: Some(sender),
            receiver: Some(receiver),
        }
    }

    fn process(&mut self, _delta: f64) {
      if let Some(receiver) = &mut self.receiver {
        if let Ok(response) = receiver.try_recv() {
          self.base_mut().emit_signal("generated_dialogue", &[Variant::from(GString::from(response))]);
        }
      }
    }
}

#[godot_api]
impl LLMCharacterBody2D {
  #[func]
  fn handle_interactions(&self) {
    let llm = Arc::clone(&self.llm);

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