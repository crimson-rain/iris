use bevy_ecs::entity::Entity;
use bevy_ecs::query::With;
use bevy_ecs::system::Query;

use super::components::{CurrentNPC, GenerationResponse};

pub fn dialogue_generation_system(
    mut query: Query<(&mut GenerationResponse, Entity), With<CurrentNPC>>,
) {
    for (mut _generation, entity) in query.iter_mut() {
        println!("Generating Dialogue for Interacting NPC: {:?}", entity);
    }
}
