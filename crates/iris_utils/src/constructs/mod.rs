use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue: String,
    pub npc: String
}
