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
    personality: GString,
    #[export]
    description: GString,
    #[export]
    relation_to_player: GString,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for NPC {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            id: GString::new(),
            role: GString::new(),
            personality: GString::new(),
            description: GString::new(),
            relation_to_player: GString::new(),
            base,
        }
    }
}

#[godot_api]
impl NPC {
    #[func]
    pub fn get_npc_info(&self) -> String {
        format!(
            "ID: {}, Role: {}, Personality: {}, Description: {}, Relation To Player: {}",
            self.id, self.role, self.personality, self.description, self.relation_to_player
        )
    }
}
