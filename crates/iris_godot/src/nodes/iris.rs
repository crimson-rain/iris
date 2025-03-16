use godot::obj::Gd;
use godot::builtin::GString;
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
        if let Some(reciever) = &mut self.channels.reciever {
            while let Ok(res) = reciever.try_recv() {
                godot_print!("{:?}", res);
            }
        }
    }

    #[func]
    pub fn generate_dialogue(&self) {
        let sender = self.channels.sender.clone();

        std::thread::spawn(move || {
            let runtime = Runtime::new().expect("Failed to Create a Tokio Runtime");
            runtime.block_on(async move {
                let mut maestro = Maestro::default();

                if let Ok(res) = maestro.conduct_dialogue_gen("Hello, How are you? Also what's the weather in Tokyo?".to_string()).await {
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
