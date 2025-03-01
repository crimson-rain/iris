use bevy_ecs::entity::Entity;
use bevy_ecs::query::With;
use bevy_ecs::system::Query;

use super::components::{CurrentNPC, GenerationResponse};

pub fn dialogue_generate_system(
    query: Query<(&mut GenerationResponse, Entity), With<CurrentNPC>>,
) {
    for (generation, entity) in query.iter() {
        println!("Generating Dialogue for Interacting NPC through {:?} \nGenerated Dialoge: {}, Choices: {:?}", entity, generation.dialogue, generation.choices);
    }
}
