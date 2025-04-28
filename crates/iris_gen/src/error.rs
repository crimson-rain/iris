//! `error.rs`
//!
//! This module is responsible for handling errors which can occur in the library
//! Handles errors from Ollama, Reqwest, Qdrant, Serde_JSON and various other errors in a 
//! error stack.

use ollama_rs::error::OllamaError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IrisGenError {
    #[error("Ollama Error: {0}")]
    OllamaError(#[from] OllamaError),

    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Qdrant Error: {0}")]
    QdrantError(#[from] qdrant_client::QdrantError),

    #[error("SerdeJSON Error: {0}")]
    SerdeJSONError(#[from] serde_json::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("No Models Found")]
    NoModelsFound,
}
