---
date: "20241112"
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/c33a68a131b32e0c60b78a948026c3824be69daa
---
## Summary  
*Refactoring folder structure, code and dependencies.*

---
## Log
#### Updated and Changed Dependencies

Added some new dependencies since last update. Shown below is a list of dependencies which I'm currently using.

- ollama-rs - Used to interact with Ollama API.
- tokio - For asynchronous programming.
- serde - Used to serialize and deserialize data.
- serde_json -  Used in addition with serde to serialize and deserialize JSON files.
- derive_more - Used for error handling.
- once_cell - Used to create singletons.

```toml
[dependencies]
# -- Godot Bindings
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }
# -- Ollama Bindings
ollama-rs = "0.2.1"
# -- Async
tokio = { version = "1.41.1", features = ["full"] }
# -- JSON Handling
serde = { version = "1.0.215", features = ["derive"] }
serde_json = "1.0.132"
# -- Others
derive_more = { version = "1.0.0-beta", features = ["from", "display"] }
once_cell = "1.20.2"
```
---

#### Renamed LLMNPC to LLM_Character_Body_2D
Renamed LLMNPC to LLM_Character_Body_2D for verbosity, but might rename this at a later point.

---

#### Restructured Error Module
Restructured error crate to integrate derive_more, as the main way to handle errors. This was taken from Jeremy Chone.

```rust
use derive_more::From;
pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, From)]
pub enum Error {
  Generic(String),
  IOError(std::io::Error),
  OllamaError(ollama_rs::error::OllamaError),
}
impl core::fmt::Display for Error {
  fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}
impl std::error::Error for Error {}
```
