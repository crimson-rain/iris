use serde::{Deserialize, Serialize};

// NPC Data Schemas
#[derive(Serialize, Deserialize)]
pub struct NPCData {
    pub id: String,
    pub title: String,
    pub npc_information: NPCInformation,
    pub notable_traits: Vec<String>,
    pub background: String,
    pub relationships: Vec<Relations>,
    pub equipment: Vec<Equipment>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NPCInformation {
    name: String,
    alias: String,
    faction: String,
    race: String,
    class: String,
    age: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Relations {
    name: String,
    role: String,
    description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Equipment {
    name: String,
    description: String,
}

// Location Schemas
#[derive(Serialize, Deserialize)]
pub struct WorldData {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub coordinates: Coordinates,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    x: f32,
    y: f32,
}
