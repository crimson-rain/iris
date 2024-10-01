// Re-export of the error crate.
pub use crate::error::Error;

// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern.
pub struct W<T>(pub T);

// Assigns Format as f.
pub use std::format as f;