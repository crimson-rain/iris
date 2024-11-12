#![allow(unused)]

mod core;
mod nodes;
mod error;

use godot::prelude::*;
use error::{Result, Error};

struct Iris;

#[gdextension]
unsafe impl ExtensionLibrary for Iris {}