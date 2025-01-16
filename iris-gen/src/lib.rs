/* FILENAME: lib.rs
 * 
 * DESCRIPTION 
 * Main Library for the Project.
 * Used to Store All Library Features.
 * 
 * NOTES
 * 
 * AUTHOR:    Rezwan Rahman (RAH22529097)
 * CREATED:   04/11/2024
 * MODIFIED:  14/11/2024
*/

pub mod error;
pub mod llm;
pub mod vec_store;

use godot::prelude::*;

struct IrisGen;

#[gdextension]
unsafe impl ExtensionLibrary for IrisGen {}

// Test for the Library
#[cfg(test)]
mod tests {

}