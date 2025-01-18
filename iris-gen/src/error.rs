//! This module provides error handling for the Iris Gen Library.

use godot::global::godot_print;
use thiserror::Error;

/// Error Type for `iris-gen`
/// Defines the different types of errors which can occur with the library.
#[derive(Error, Debug)]
pub enum IrisError {
    #[error("Ollama Error: {0}")]
    OllamaGenerationError(#[from] ollama_rs::error::OllamaError),
    #[error("Failed to Parse JSON into Dialogue: {0}")]
    FailedToSerialize(#[from] serde_json::error::Error),
}

impl Drop for IrisError {
    fn drop(&mut self) {
        godot_print!("Error: {}", self);
    }
}

#[cfg(test)]
mod tests {
    use serde::de::Error;
    use super::IrisError;

    #[test]
    fn test_auto_godot_print() {
        let _error = IrisError::FailedToSerialize(serde_json::Error::custom("Test Error"));
    }
}
