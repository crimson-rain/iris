/* FILENAME: llm_npc/mod.rs
 *
 * DESCRIPTION
 * The LLMCharacterBody2D is the Node Abstraction for Godot, which provides, 
 * LLM generated responses based on the world and user prompt.
 * 
 * There are two systems in place, one designed to generate dialogue interactions and
 * one to generate quests. 
 * 
 * NOTES
 * Implement RAG, for Long Term Memory for Large Language Model.
 * Implement Memory(History) for Short Term Memory.
 * 
 * RESOURCES:
 * Godot-Rust API Bindings: https://godot-rust.github.io/book/
 * Tokio Channels: https://tokio.rs/tokio/tutorial/channels
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
    #[func]
    fn handle_interactions(&self) {
        let llm = Arc::clone(&self.llm);
        let sender = self.sender.clone();

        let prompt = "How do I break into a Ford? Car".to_string();

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