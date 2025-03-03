use godot::{builtin::GString, classes::{CharacterBody2D, ICharacterBody2D}, obj::Base, prelude::{godot_api, GodotClass}};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct NPC {
    #[export]
    id: GString,
    #[export]
    role: GString,

    base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for NPC {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self { 
            id: GString::new(), 
            role: GString::new(),
            base,
        }
    }
}