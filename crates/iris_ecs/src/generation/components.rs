use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct CurrentNPC;

#[derive(Component)]
pub struct GenerationResponse {
    pub dialogue: String,
    pub choices: Vec<String>,
}
