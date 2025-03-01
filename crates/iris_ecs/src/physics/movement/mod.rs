pub mod components;
pub mod systems;

#[cfg(test)]
mod tests {
    use super::components::{Position, Velocity};
    use super::systems::movement_system;
    use bevy_ecs::prelude::*;

    #[test]
    fn test_movement_system() {
        let mut world = World::new();

        let entity = world
            .spawn((Position { x: 0.0, y: 0.0 }, Velocity { dx: 2.0, dy: 3.0 }))
            .id();

        let mut schedule = Schedule::default();
        schedule.add_systems(movement_system);
        schedule.run(&mut world);

        let pos = world.get::<Position>(entity).unwrap();

        assert_eq!(pos.x, 2.0);
        assert_eq!(pos.y, 3.0);
    }
}
