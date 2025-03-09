use godot::classes::{INode, Node};
use godot::global::godot_print;
use godot::obj::Base;
use godot::prelude::{GodotClass, godot_api};
use iris_gen::agent::maestro::Maestro;

#[derive(GodotClass)]
#[class(base=Node)]
struct Iris {
    maestro: Maestro,
    base: Base<Node>,
}

#[godot_api]
impl INode for Iris {
    fn init(base: Base<Node>) -> Self {
        Iris {
            maestro: Maestro::default(),
            base 
        }
    }
    
    fn process(&mut self, _delta: f64) {

    }
}

#[godot_api]
impl Iris {
    // FIXME
    // Abstract the Function and turn it into a signal.
    #[func]
    pub fn generate_dialogue(&self) {
        godot_print!("DIALOGUE: {}", self.maestro.conduct_dialogue_gen());
    }

    #[func]
    pub fn generate_quest(&self) {
        godot_print!("QUEST: {}", self.maestro.conduct_quest_gen());
    }
}
