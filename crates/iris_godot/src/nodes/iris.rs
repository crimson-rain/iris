use godot::obj::Gd;
use godot::builtin::GString;
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
}

#[godot_api]
impl Iris {
    #[func]
    pub fn generate_dialogue(&self) {
        godot_print!("generate_dialogue called");
    }

    #[func]
    pub fn generate_quest(&self) {
        godot_print!("QUEST: {}", self.maestro.conduct_quest_gen());
    }

    #[signal]
    fn dialogue_generated(response: GString);
}
