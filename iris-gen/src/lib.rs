//! ### Overview
//! The purpose of this library is to generate contextually aware and emergent dialogue
//! for NPCs. The library is primarily anchored towards the Godot Engine.
//!
//! This library takes advantage of the Ollama API to generate contextually aware dialgoues,
//! as well as create embeddings to store in a vector-store for RAG.
//!
//! ### Future Works / Features
//! - Implement Quest Generation
//! - Hexagonal Architecture (Performance Cost If Using APIs)

pub mod agent;
pub mod error;
pub mod llm_character;
pub mod memory;
pub mod schemas;
pub mod utils;
pub mod vector_store;

use godot::prelude::*;

struct IrisGen;

#[gdextension]
unsafe impl ExtensionLibrary for IrisGen {}
