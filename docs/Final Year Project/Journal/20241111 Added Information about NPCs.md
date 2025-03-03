---
date: "20241111"
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/1fb5bb44ace71fba55e31c6e0b2a6922a0d567b1
---
## Summary  
Added names, profession and descriptions to NPCs.

---
## Log
#### Added Fields ID, Profession and Description for NPCs
Added additional fields to `LLMCharacterBody2D` such as the id which stores the NPCs name, profession the job of the NPC i.e. blacksmith, chef, and the description of the NPC and general characteristics. All of these fields are directly exported in Godot which promotes ease of use.

```rust
#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct LLMCharacterBody2D {
    // Character Exports
    #[export]
    id: GString,
    #[export]
    profession: GString,
    #[export]
    description: GString,
    memory: Vec<String>,
    base: Base<CharacterBody2D>,
    llm: Arc<LLM>,
    sender: Option<Sender<String>>,
    receiver: Option<Receiver<String>>,
}
```