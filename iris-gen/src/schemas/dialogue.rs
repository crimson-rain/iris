//! This module provides functionality for creating and managing NPC memories.

use crate::{error::IrisError, utils::parse_json::parse_json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue: String,
    pub npc: String,
    pub choices: Vec<String>,
}

impl TryFrom<&str> for Dialogue {
    type Error = IrisError;

    fn try_from(data: &str) -> Result<Self, IrisError> {
        let format_json = parse_json(data)?;
        serde_json::from_str::<Dialogue>(format_json.as_str())
            .map_err(IrisError::FailedToSerialize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_dialogue_test() {
        let data = r#"
        {
            "dialogue": "Welcome, traveller! What brings you to our village?",
            "npc": "Elder Rowan",
            "choices": [
                "I'm just passing through.",
                "I'm looking for someone.",
                "Can you tell me more about this place?"
            ]
        }
        "#;

        let dialogue = Dialogue::try_from(data).expect("Failed to Deserialize Dialogue");

        assert_eq!(
            dialogue.dialogue,
            "Welcome, traveller! What brings you to our village?"
        );
        assert_eq!(dialogue.npc, "Elder Rowan");
        assert_eq!(
            dialogue.choices,
            vec![
                "I'm just passing through.",
                "I'm looking for someone.",
                "Can you tell me more about this place?"
            ]
        );
    }

    #[test]
    fn fail_to_serialize_test() {
        let data = r#"
        {
            "dialogue": "Welcome, traveller! What brings you to our village?"
        }
        "#;

        let result = Dialogue::try_from(data);
        assert!(result.is_err());
    }
}
