---
date: 2024-09-20
tags:
  - Documentation
  - FYP
cssclasses: []
commit-url: https://github.com/Crimson-Rain/iris/commit/61997f0399a837247501446d717957575f3ef1bb
---
# Project Setup

Completed the Project Setup
File Structure After Setting Up the GitHub Project

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


`docs` - Stores Design Documentations
`iris` - Stores the Godot Game Files.
`iris-api` - Stores the Code for API, which is built using Rust Programming Language.

---

I've also setup the project so the library binds to Godot and uses the Godot API.
Godot provides bindings to other programming languages to allow developers to create external features which would be hard to do using Godot.

I'm currently using [godot-rust](https://godot-rust.github.io/) which is rust-bindings for Godot. Following the setup process in their [godot-rust book](https://godot-rust.github.io/book/intro/setup.html)

These are the steps to get setup which I followed
1. First install rust and toolchains such as clippy etc.
2. Create a new Rust project using `cargo new iris-api --lib`
3. The next step was to make sure that the crate compiles to a dynamic C library to meet godots requirements. 
```toml
[package]
name = "iris-api"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
# -- Godot Bindings
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }
```

4. After completing the previous step, add the godot-rust library, this is done using Rust package manager called Cargo, running this command `cargo add godot`
5. Now we must wire-up the API/Library to Godot using `.gdextension` file inside a Godot subfolder. 
```ini
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.3
reloadable = true

[icons]
LLMCharacterBody2D = "res://Assets/Icons/ai-chip.svg"

[libraries]
windows.debug.x86_64 = "res://../iris-api/target/debug/iris_api.dll"
```

6. Finally we can compile the program, and it will successfully link to Godot.