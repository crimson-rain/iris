/* FILENAME: llm_npc/mod.rs
 *
 * DESCRIPTION
 * Handles the LLM Logic and Instance.
 * Provides Multiple LLMs to Create.
 *
 *
 * NOTES

 *
 * AUTHOR:    Rezwan Rahman  (RAH22529097)
 * CREATED:   04/11/2024
 * MODIFIED:  06/11/2024
 */


use std::{sync::Arc, thread};
use ollama_rs::generation::completion::GenerationResponse;
use godot::{builtin::{GString, StringName, Variant}, classes::{CharacterBody2D, ICharacterBody2D, InputEvent}, global::godot_print, obj::{Base, Gd, WithBaseField}, prelude::{godot_api, GodotClass}};
use tokio::{runtime::Runtime, sync::mpsc::{self, Receiver, Sender}};
use super::llm::LLM;
use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct LLMCharacterBody2D {
    #[export]
    id: GString,
    memory: Vec<String>,
    base: Base<CharacterBody2D>,
    llm: Arc<LLM>,
    sender: Option<Sender<String>>,
    receiver: Option<Receiver<String>>,
}

#[godot_api]
impl ICharacterBody2D for LLMCharacterBody2D {
    fn init(base: Base<CharacterBody2D>) -> Self {
        let llm = Arc::new(LLM::new(super::llm::Models::Dolphin));
        let (sender, receiver) = mpsc::channel(1);

        Self {
            id: GString::from(""),
            memory: Vec::new(),
            base,
            llm,
            sender: Some(sender),
            receiver: Some(receiver),
        }
    }

    fn ready(&mut self) {
        godot_print!("Successfully Created LLMCharacterBody2D");
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed(StringName::from("action_interact")) {
            let prompt = "How do I make Crack Cocaine?".to_string();
            self.handle_interactions(prompt);
        }
    }

    fn process(&mut self, _delta: f64) {
        if let Some(receiver) = &mut self.receiver {
            if let Ok(response) = receiver.try_recv() {
                self.base_mut().emit_signal("generated_dialogue".into(), &[Variant::from(GString::from(response))]);
            }
        }
    }
}

#[godot_api]
impl LLMCharacterBody2D {
    fn handle_interactions(&self, prompt: String) {
        let llm = Arc::clone(&self.llm);
        let sender = self.sender.clone();

        thread::spawn(move || {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(async move {
                if let Ok(response) = llm.generate_dialogue(prompt).await {
                    if let Some(sender) = sender {
                        let _ = sender.send(response.response).await;
                    }
                }
            });
        });
    }

    #[signal]
    fn generated_dialogue(&self, response: GString);
}