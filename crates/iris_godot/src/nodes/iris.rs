use godot::classes::{INode, Node};
use godot::global::godot_print;
use godot::obj::Base;
use godot::prelude::{GodotClass, godot_api};

#[derive(GodotClass)]
#[class(base=Node)]
struct Iris {
    base: Base<Node>,
}

// Godot related functions
#[godot_api]
impl INode for Iris {
    fn init(base: Base<Node>) -> Self {
        Iris { base }
    }

    fn process(&mut self, _delta: f64) {}
}

// Non-Godot related functions
#[godot_api]
impl Iris {
    pub fn start_dialogue(&self) {
        godot_print!("Dialogue Started");
    }
}
