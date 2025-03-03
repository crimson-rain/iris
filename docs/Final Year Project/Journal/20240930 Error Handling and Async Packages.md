---
date: 2024-09-30
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/crimson-rain/iris/commit/758d7e4c2ea60af5b19ab32f2eee161e47807958
---
## Summary  
*Added packages to help with error handling and asynchronous programming.*

---
## Log
#### Added `thiserror` for Error Handling
thiserror trivializes error handling in rust. This is done by using an error stacks. Where all errors which can be defined in your library can be defined in the error stack. 

**Example of thiserror**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

---
#### Added `tokio` for Asynchronous Programming
tokio is an asynchronous runtime library implemented in rust. Primarily used for writing network applications, I have used this to handle the waiting and blocking of tasks when generating dialogues from the large language model.

---
## Dependencies and Libraries
`Cargo.toml`
```toml
[package]
name = "iris-api"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }
thiserror = "1.0.64"
tokio = "1.40.0"

[dev-dependencies]
anyhow = "1.0.89"
```




---
