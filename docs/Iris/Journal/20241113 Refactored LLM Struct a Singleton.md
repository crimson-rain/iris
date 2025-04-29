---
date: "20241113"
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/b6ff5d21809566f76012fc65829c1471e9428aa6
---
## Summary  
*Refactored the LLM Struct to become a singleton to save system resources.*

---
## Log
#### Converted LLM Struct to a Singleton
Converted LLM struct to a singleton using once_cell, this allows to save on system resources and improve general performance when there are multiple NPCs which need to call the LLM without creating a new instance of the LLM struct. By default `Mistral7B` is used, at a later point a more robust and dynamic way should be used to instantiate models out of scope.

```rust
pub static LLM_INSTANCE: Lazy<Arc<LLM>> = Lazy::new(|| {
	Arc::new(LLM::new(Models::Mistral7B))
});

pub fn get_instance() -> Arc<LLM> {
	Arc::clone(&LLM_INSTANCE)
}

```