pub mod components;
pub mod systems;

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use components::{CurrentNPC, GenerationResponse};
    use systems::dialogue_generate_system;

    #[test]
    fn test_generation() {
        let mut world = World::new();

        let entity = world
            .spawn((
                CurrentNPC,
                GenerationResponse {
                    dialogue: "Hello, World!".to_string(),
                    choices: vec!["Hello".to_string(), "Goodbye".to_string()],
                },
            ))
            .id();

        let mut schedule = Schedule::default();

        schedule.add_systems(dialogue_generate_system);
        schedule.run(&mut world);

        let generated = world.get::<GenerationResponse>(entity).unwrap();

        assert_eq!(generated.dialogue, "Hello, World!");
        assert_eq!(generated.choices, vec!["Hello".to_string(), "Goodbye".to_string()]);
    }
}
