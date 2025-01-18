//! This module provides functionality for creating and managing NPC memories.

pub mod error;
pub mod llm;
pub mod llm_character;
pub mod memory;
pub mod model;
pub mod vec_store;

use godot::prelude::*;

struct IrisGen;

#[gdextension]
unsafe impl ExtensionLibrary for IrisGen {}

// Test for the Library
#[cfg(test)]
mod tests {}
