---
date: 2024-10-27
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: Commit Hash or URL
---
## Summary  
*Created Dialogue Manager inside Rust to handle generated dialogue from the LLM.*

---
## Log
#### Created Dialogue Manager Struct in Rust
Created a dialogue manager to manage the generated dialogue from the LLM, this allows an application layer for the LLM to interact with the game and vice versa.

```rust
#[derive(GodotClass)]
#[class(base=Node)]
struct DialogueManager {
  model: String,
  system: String,
  runtime: Runtime,
  ollama: Ollama
}
```