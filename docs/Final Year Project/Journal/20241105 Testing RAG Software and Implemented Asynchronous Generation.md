---
date: 2024-11-05
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/1fb5bb44ace71fba55e31c6e0b2a6922a0d567b1
---
## Summary  
*Implemented asynchronous generation for Dialogue and also tested RAG.*

---

## Log

#### Testing Implementation of RAG
Experimented with RAG being implemented with the LLM. This won't be implemented until a good vector database is found and implemented.

---

#### Implemented Dialogue Generation
Successfully implemented dialogue generation into the game. But an error occurs as the dialogue generation is asynchronous and requires the await keyword. This would cause the main thread to be blocked, and required multi-threading so that the main game thread isn't halted or waiting.

---
#### Added More LLMs
I've added additional support for other LLMs such as Gemma7B, Mistral7B and Mixtral. Instead of requiring multiple different LLMs, maybe I can create a function which can retrieve all available LLMs from Ollama API, and see which ones are locally installed and can be ran.

---

## Challenges & Solutions  
| Issue Encountered                | Solution Applied     | Notes                                                                                                                                                                 |
| -------------------------------- | -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Blocking of Main Thread          | Created a New Thread | Creating a new thread would block the new thread, without blocking the main game thread preventing other processes from halting or waiting.                           |
| Transferring Data Across Threads | MPSC Channels        | Using MPSC Channels or otherwise known as multi-producer single consumer channels, we can send data between threads. Alternatives is using one-shot channels instead. |
