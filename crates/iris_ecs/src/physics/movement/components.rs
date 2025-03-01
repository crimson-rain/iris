use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Component)]
pub struct Acceleration {
    pub ddx: f32,
    pub ddy: f32,
}
