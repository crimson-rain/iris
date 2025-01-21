//! This module provides error handling for the Iris Gen Library.
//!
//! Makes use of thiserror to create an error enum for all errors which can occur in the library.

use godot::global::godot_error;
use thiserror::Error;

/// Error Type for `iris-gen`
/// Defines the different types of errors which can occur with the library.
#[derive(Error, Debug)]
pub enum IrisError {
    #[error("Ollama Error: {0}")]
    OllamaGenerationError(#[from] ollama_rs::error::OllamaError),
    #[error("Failed to Parse JSON into Dialogue: {0}")]
    FailedToSerialize(#[from] serde_json::error::Error),
    #[error("I/O Error: {0}")]
    IOError(std::io::Error),
}

impl Drop for IrisError {
    fn drop(&mut self) {
        godot_error!("Error: {}", self);
    }
}
