//! This module provides error handling for the Iris Gen Library.

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

#[cfg(test)]
mod tests {}
