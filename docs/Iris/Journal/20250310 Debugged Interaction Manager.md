---
date: YYYY-MM-DD
tags:
  - Documentation
  - FYP
  - Debug
cssclasses: 
commit-url: Commit Hash or URL
---
## Summary  
*A brief overview of what was accomplished in this entry (1-3 sentences).*

---
## Log
#### Debugged Interaction Manager
* When the interaction manager is sorted values by nearest, it can return a nullptr if the player is moving to quickly, this will result in a crash.

```go
if player == null or area_one == null or area_two == null:
	return false
```

This is simply fixed by checking all values aren't null.

We can actually remove everything but the player check for null.
This could be some technical debt, as I don't really understand how a instantiated player can cause this.
The interaction manager becomes a little more laggy and choppy and would require you to fully exit a NPCs interaction range for it to work.


---

## Challenges & Solutions  
| Issue Encountered | Solution Applied | Notes |
|------------------|----------------|------|
| Example Issue 1 | Fix applied (e.g., updating a function) | Explanation/Link to commit |
| Example Issue 2 | Solution used (e.g., restructured file paths) | Additional notes |

---

## Code Snippets (If Applicable)
```rust
// Example Rust code
fn example() {
    println!("Hello, world!");
}
```
