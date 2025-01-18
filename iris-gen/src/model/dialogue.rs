//! This module provides functionality for creating and managing NPC memories.

use crate::error::IrisError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue: String,
    pub npc: String,
    pub choices: Vec<String>,
}

impl TryFrom<&str> for Dialogue {
    type Error = IrisError;

    fn try_from(data: &str) -> Result<Self, Self::Error> {
        serde_json::from_str::<Dialogue>(data).map_err(IrisError::FailedToSerialize)
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

    #[should_panic]
    #[test]
    fn fail_to_serialize_test() {
        let data = r#"
        {
            "dialogue": "Welcome, traveller! What brings you to our village?"
        }
        "#;

        let result = Dialogue::try_from(data);

        match &result {
            Err(IrisError::FailedToSerialize(err)) => {
                assert_eq!(err.to_string(), "missing field `npc` at line 4 column 9");
            }
            _ => panic!("Expected FailedToSerialize error, but got {:?}", result),
        }
    }
}
