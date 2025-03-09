//! `error.rs`
//!
//!

use ollama_rs::error::OllamaError;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum IrisGenError {
    #[error("Ollama Error: {0}")]
    OllamaError(#[from] OllamaError),
    
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("No Models Found")]
    NoModelsFound,
}
