use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct CurrentNPC(pub Entity);

#[derive(Component)]
pub struct GenerationResponse {
    pub dialogue: String,
    pub options: Vec<String>,
}
