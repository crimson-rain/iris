---
date: "20241114"
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: Commit Hash or URL
---
## Summary  
*A brief overview of what was accomplished in this entry (1-3 sentences).*

---
## Log
#### Dialogue Struct for Serialization and Deserialization
Created and implemented a basic dialogue struct which will parse JSON objects, into rust structs. This allows for more complex operations.
The JSON object should contain, dialogue, NPC information and a vector of strings representing choices.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub dialogue: String,
    pub npc: String,
    pub choices: Vec<String>,
}
```

