use std::sync::Arc;
use std::alloc::System;
use godot::{builtin::{GString, StringName, Variant}, classes::{CharacterBody2D, ICharacterBody2D, InputEvent}, global::godot_print, obj::{Base, Gd, WithBaseField}, prelude::{godot_api, GodotClass}};
use ollama_rs::generation::completion::{request::GenerationRequest, GenerationResponse};
use tokio::{runtime::Runtime, time::error};
use tokio::sync::mpsc;
use crate::prelude::*;
use super::llm::{Models, SystemPrompt};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct SmartNPC {
    #[export]
    id: GString,
    memory: Vec<String>,
    base: Base<CharacterBody2D>,
    runtime: Arc<Runtime>,
    receiver: Option<mpsc::Receiver<String>>,
}

/* TODO: 
 *
 * - Comment the Code
 * - Implement Short Memory
 * - Implement RAG (Long Memory)
 */
#[godot_api]
impl ICharacterBody2D for SmartNPC {

    // Create SmartNPC Instance
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            id: GString::from(""),
            memory: vec![],
            base,
            runtime: Arc::new(Runtime::new().expect("Failed to Create Tokio Runtime")),
            receiver: None,
        }
    }

    fn ready(&mut self) {
        let callable = self.base_mut().callable("success_generation");
        self.base_mut().connect("dialogue_generated".into(), callable);
    }
    
    fn physics_process(&mut self, _delta: f64) {
        if let Some(receiver) = &mut self.receiver {
            match receiver.try_recv() {
                Ok(response) => {
                    self.base_mut().emit_signal("dialogue_generated".into(), &[Variant::from(response.clone())]);

                    if self.memory.len() >= 5 {
                        self.memory.remove(0);
                    }

                    self.memory.push(response.clone());

                    godot_print!("{}: {}", &self.id, response);
                },
                Err(_) => {}
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed(StringName::from("action_interact")) {
            let (sender, receiver) = mpsc::channel(1);
            self.receiver = Some(receiver);
            
            let prompt = "What colour is the sky?".to_string();
            let id = self.id.clone();
            let runtime = Arc::clone(&self.runtime);
            
            // TODO: do I have to write this disgusting ass code?
            runtime.spawn(async move {
                match SmartNPC::generate_dialogue(&prompt).await {
                    Ok(response) => {
                        let _ = sender.send(response.response).await;
                    },
                    Err(e) => {
                        let _ = sender.send(format!("Error: {:?}", e)).await;
                    }
                }
            });
        }
    }
}

// Should this be abstracted towards the LLM Layer? Perhaps this would be more convenient if it was?
impl SmartNPC {
    // Generate Dialogue
    pub async fn generate_dialogue(prompt: &str) -> Result<GenerationResponse> {
        let model = Models::Mistral7B.initialize_model();

        // Format the Prompt
        let formatted_prompt = format!(
            "{}{}",
            SystemPrompt::DialogueSystem.get_system_prompt(),
            prompt
        );

        // Generate Response from Model
        let response = model.ollama.generate(
            GenerationRequest::new(model.model.get_model(), formatted_prompt)
        ).await?;

        Ok(response)
    }

    // Generate Quest
    pub async fn generate_quest() -> Result<GenerationResponse> {
        let model = Models::Mistral7B.initialize_model();
        let formatted_prompt = format!("{}{}", SystemPrompt::QuestSystem.get_system_prompt(), "No Prompt");

        let response = model.ollama.generate(
            GenerationRequest::new(model.model.get_model(), formatted_prompt)
        ).await?;

        Ok(response)
    }
}

// Handle the Signal in a Dialogue Box inside Godot
#[godot_api]
impl SmartNPC {
  #[signal]
  fn dialogue_generated();

  #[func]
  fn success_generation(&mut self, response: Variant) {
    godot_print!("Generated Dialogue");
  }
}
