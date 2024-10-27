use godot::prelude::*;

mod error;
mod prelude;
mod utils;
mod core;

struct Iris;

#[gdextension]
unsafe impl ExtensionLibrary for Iris {}