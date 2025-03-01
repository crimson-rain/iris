use bevy_ecs::system::Query;

use super::components::*;

pub fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }
}

pub fn acceleration_system(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut vel, acc) in &mut query {
        vel.dx += acc.ddx;
        vel.dy += acc.ddy;
    }
}
