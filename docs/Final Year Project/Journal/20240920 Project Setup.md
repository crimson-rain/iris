---
date: 2024-09-20
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/61997f0399a837247501446d717957575f3ef1bb
---
## Summary  
*A structured setup of the project to ensure modularity maintainability and scalability.*

---
## Log

#### Created GitHub Repository
Created a GitHub repository for VCS and a central access point for the project.
Using VCS Enables:
- Rollbacks to previous versions in case of errors or regressions.
- Branching to work on new features or bug fixes independently.
- History for documentation and debugging purposes.

Link to Repository: [Link](https://github.com/crimson-rain/iris)

---
#### Created Godot Project 
Created Godot Project. Godot will be the game engine that will be used in this project. 

**Rationale**
- Simple and fast to learn.
- Extensive support and has asset libraries.
- External scripts and programming support.

---

#### Created Rust Project
The rust library is the heart of the project and will contain the all the logic needed to generate dialogue. 

**Rationale for Picking Rust**
- Rust is has up to date bindings for Godot.
- Rust has a library to interact with Ollama API.
- Performant programming language.

---

#### Setup Libraries and Integrated Rust Library into Godot
**Steps Taken to Setup and Integrate Rust Library into Godot**
1. Installed Rust and toolchains.
2. Create a new rust library using `cargo new iris-api --lib`.
3. Compile cargo library to ensure that it creates a dynamic C library.
4. Add godot-rust, for binding for godot.
5. Wire-up the rust library to godot using a `.gdextension` file inside.
6. Compile program and check godot has successfully linked to the project.

This was done following a guide found on [godot-rust book](https://godot-rust.github.io/book/intro/setup.html)

---

#### Updated Dependencies

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
```

`iris-api.gdextension`
```toml
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.3
reloadable = true

[icons]
LLMCharacterBody2D = "res://Assets/Icons/ai-chip.svg"

[libraries]
windows.debug.x86_64 = "res://../iris-api/target/debug/iris_api.dll"
```

---

## Finished File Structure

```
docs 
├── .gitkeep 
iris 
├── Assets 
│ └── .gitkeep 
├── Common 
│ └── .gitkeep 
├── Config 
│ └── .gitkeep 
├── Entities 
│ └── .gitkeep 
├── Externals 
│ └── iris-api.gdextension 
├── Levels 
│ └── .gitkeep 
├── Utilities 
│ └── .gitkeep 
├── project.godot 
iris-api 
├── Cargo.toml 
├── src
│ └── lib.rs
```
