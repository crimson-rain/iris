use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue: String,
    pub npc: String,
}
