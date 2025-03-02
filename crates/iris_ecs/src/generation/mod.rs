use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct GenerationTag;

#[derive(Component)]
pub struct GenerationTarget(pub Entity);
