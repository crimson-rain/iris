//! iris_godot
//! 
//! `iris_godot` is the core and heart of this project allowing us to interact with the Godot Game Engine
//! using FFIs and Bindings through the godot-rust crate. 
//! 
//! This library is responsible for connecting all the components together and have these components accessible in godot.

use godot::prelude::*;

struct Iris;

#[gdextension]
unsafe impl ExtensionLibrary for Iris {}