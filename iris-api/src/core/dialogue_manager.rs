use godot::{
  builtin::StringName,
  classes::{INode, InputEvent, Node},
  global::godot_print,
  obj::{Base, Gd},
  prelude::{godot_api, GodotClass},
};

use ollama_rs::{generation::completion::{request::GenerationRequest, GenerationResponse}, Ollama};
use tokio::runtime::Runtime;

use crate::prelude;
use prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct DialogueManager {
  model: String,
  system: String,
  runtime: Runtime,
  ollama: Ollama
}

const MODEL: &str = "dolphin-llama3";
const MODEL_SYSTEM_PROMPT: &str = "
  You are to act as a Non-Player Character Inside a Game.
  You are given information about the character such as their name, profession and general details about the character as well as their relationships with other characters.
  You are also given a general description of the characters traits etc.
  You must only talk to the character and nothing more.

  When the Player Interacts with you you are to act as the prompt given.
  Make the dialogue short and concise.
  
  Do Not Mention That you are Dolphin.
";


#[godot_api]
impl INode for DialogueManager {
  fn init(_base: Base<Node>) -> Self {
      godot_print!("Successfully Compiled and Created DialogueManager Node");

      let runtime = Runtime::new().expect("Failed to create Tokio runtime");

      Self {
          model: MODEL.to_string(),
          system: MODEL_SYSTEM_PROMPT.to_string(),
          runtime,
          ollama: Ollama::default()
      }
  }

  fn input(&mut self, event: Gd<InputEvent>) {
      if event.is_action_pressed(StringName::from("action_interact")) {
          let character_data = String::from("
              Name: John Hawthorne
              Profession: Soldier
              Description:
              You are a bit of a brute, but a kind one. 
              Making sure everyone is safe and always being there to help.
              You might look scary but you are a soft and kind person, but can rise to the occasion
              if danger is headed your way.

              Relation with Player: 
              Neutral

              Other Relations:
              None
          ");

          let prompt = String::from("
            I'm looking for a mighty weapon!
          ");

          let model = self.model.clone();
          let system = self.system.clone();
          let response = self.runtime.block_on(async {
              self.generate_dialogue(model, system, character_data, prompt).await
          }).unwrap();

          godot_print!("{}", response.response);
      }
  }
}

impl DialogueManager {
  pub async fn generate_dialogue(&self, model: String, system: String, character_data: String, prompt: String) -> Result<GenerationResponse> {
      let full_prompt = f!("{}{}{}", system, character_data, prompt);
      
      let response = self.ollama.generate(GenerationRequest::new(model, full_prompt)).await?;
      
      Ok(response)
  }
}