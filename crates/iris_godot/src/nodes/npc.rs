use godot::builtin::GString;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::obj::Base;
use godot::prelude::{GodotClass, godot_api};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct NPC {
    #[export]
    id: GString,
    #[export]
    role: GString,
    #[export]
    description: GString,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for NPC {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            id: GString::new(),
            role: GString::new(),
            description: GString::new(),
            base,
        }
    }
}
