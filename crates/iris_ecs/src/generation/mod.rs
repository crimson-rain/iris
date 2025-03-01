pub mod components;
pub mod systems;

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use components::{CurrentNPC, GenerationResponse};
    use systems::dialogue_generation_system;

    #[test]
    fn test_generation() {
        let mut world = World::new();

        let _entity = world
            .spawn((
                CurrentNPC,
                GenerationResponse {
                    dialogue: "Hello, World!".to_string(),
                    choices: vec!["Hello".to_string(), "Goodbye".to_string()],
                },
            ))
            .id();

        let mut schedule = Schedule::default();

        schedule.add_systems(dialogue_generation_system);
        schedule.run(&mut world);
    }
}
