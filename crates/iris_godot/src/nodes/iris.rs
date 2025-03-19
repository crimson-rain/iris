use godot::obj::{Gd, WithBaseField};
use godot::builtin::{GString, Variant};
use godot::classes::{INode, Node};
use godot::global::godot_print;
use godot::obj::Base;
use godot::prelude::{GodotClass, godot_api};
use iris_gen::agent::maestro::Maestro;
use iris_utils::asyn::channels::Channels;
use tokio::runtime::Runtime;

#[derive(GodotClass)]
#[class(base=Node)]
struct Iris {
    channels: Channels<String>,
    base: Base<Node>, 
}

#[godot_api]
impl INode for Iris {
    fn init(base: Base<Node>) -> Self {
        Iris {
            channels: Channels::default(),
            base
        }
    }

    fn process(&mut self, _delta: f64) {
        self.process_generated_dialogue();
    }
}

#[godot_api]
impl Iris { 
    #[func]
    fn process_generated_dialogue(&mut self) {
        let mut dialogue_arr = Vec::new();

        if let Some(receiver) = &mut self.channels.reciever {
            while let Ok(res) = receiver.try_recv() {
                dialogue_arr.push(res);
            }
        }

        for dialogue in dialogue_arr {
            self.base_mut().emit_signal("dialogue_generated", &[Variant::from(dialogue)]);
        }
    }

    #[func]
    pub fn generate_dialogue(&self, prompt: String, npc_data: String) {
        let sender = self.channels.sender.clone();

        std::thread::spawn(move || {
            let runtime = Runtime::new().expect("Failed to Create a Tokio Runtime");
            runtime.block_on(async move {
                let mut maestro = Maestro::default();
                
                // Format NPC Data and Prompt and Generate Dialogue
                let formatted_prompt = format!("Prompt: {}, NPC: {}", prompt, npc_data);
                if let Ok(res) = maestro.conduct_dialogue_gen(formatted_prompt).await {
                    if let Some(sender) = sender {
                        let _ = sender.send(res).await;
                    }
                }
            });
        });
    }

    #[func]
    pub fn generate_quest(&self) {
        godot_print!("Hello, User");
    }

    #[signal]
    fn dialogue_generated(response: GString);
}
