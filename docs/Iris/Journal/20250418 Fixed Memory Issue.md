---
date: 2025-04-20
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: Commit Hash or URL
---
## Summary  
*There was  a bug a problem where the memory for NPC would be dropped after it was cloned.
This was simply solved by moving Chat History from the Maestro struct to Iris. Allowing for Persistent Storage.
The reason why this was a challenge was because a new thread would be spawned to handle the calling of the LLM. 
To avoid blocking the main thread which runs the game.*

---
## Log
#### Identify the Problem
First Identified what the problem was and why it was occurring.
* I found the cause was that memory wasn't being updated for the main Maestro class.
- Memory would be dropped and wouldn't be persistent, so I moved to the Memory from Maestro to Iris.
- I added additional unit tests to ensure that memory is consistent as well as persistent.

---

## Challenges & Solutions  
| Issue Encountered                                                                                                               | Solution Applied                                                                                                                  | Notes                                                                                          |
| ------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| Handling Multithreaded Operations for Memory,<br>As a new thread was being spawned, memory would not be persistent and updated. | The fix which was applied was moving the Chat History from Maestro to Iris, and cloning the value and manually updating it later. | [Commit](https://github.com/crimson-rain/iris/commit/a0ccc2f6b773b75c7d757c2bcbd79f5abfb08058) |
| Maestro would drop the new history as its also cloned                                                                           | The solution above also solved this problem.                                                                                      |                                                                                                |


---

## Struct of Iris with Chat History 

```rust
#[derive(GodotClass)]
#[class(base=Node)]
struct Iris {
    channels: Channels<String>,
    base: Base<Node>,
    maestro: Maestro,
    history: Vec<ChatMessage>,
}
```

## Struct of Iris with Chat History 

```rust
#[func]
pub fn generate_dialogue(&mut self, prompt: String, npc_data: String) {
	let sender = self.channels.sender.clone();
	let mut maestro = self.maestro.clone();
	let mut history = self.history.clone();

	self.history.push(ChatMessage::new(
		ollama_rs::generation::chat::MessageRole::User,
		prompt.clone(),
	));

	std::thread::spawn({
		move || {
			let runtime = Runtime::new().expect("Failed to Create a Tokio Runtime");
			runtime.block_on(async move {
				let formatted_prompt = format!("Prompt: {}, NPC: {}", prompt, npc_data);

				match maestro.conduct_dialogue_gen(formatted_prompt, &mut history).await {
					Ok(res) => {
						if let Some(sender) = sender {
							let _ = sender.send(res).await;
						}
					}
					Err(err) => {
						godot_error!("Failed to Generate Dialogue: {:?}", err);
					}
				}
			});
		}
	});
}
```
