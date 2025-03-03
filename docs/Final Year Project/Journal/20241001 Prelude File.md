---
date: 2024-10-01
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/6550527ff47ceff06455d6764932486c571a7b53
---
## Summary  
*Created prelude file to import commonly used functions and utils.*

---
## Log
#### Setting up Prelude File
A prelude file was setup in the source directory. The purpose of the prelude file is to import and use common functions and utils. Some examples are the error and result types which are found all over rust. General tips suggest that the prelude file should be kept small.

**prelude.rs**
```rust
// Re-export of the error crate.
pub use crate::error::Error;

// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern
pub struct W<T>(pub T);

pub use std::format as f;   
```

The setup was taken from [Jeremy Chone](https://jeremychone.com/).