pub mod error;

use godot::prelude::*;

struct IrisGen;

#[gdextension]
unsafe impl ExtensionLibrary for IrisGen {}
