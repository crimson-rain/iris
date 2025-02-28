---
date: 
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/1fb5bb44ace71fba55e31c6e0b2a6922a0d567b1
---
Today I was testing and implementing RAG software. But this is still a work in progress, and can't be fully implemented until I find a library which will help me implement vector databases.

I've also started to implement the generation of dialogues in the game, and encountered a problem where the generation of dialogue will block the main thread, I resolved this by coding a separate thread to run the generation process. But had to use the tokio package for asynchronous programming, this allows for the await keyword in rust, meaning that we can await the process to finish in a separate thread without blocking the main game.

I've also added some additional LLM Support, I will improve this such that people may toggle LLMs inside the game directly rather than managing them inside a config file in the API.

The LLMs which I have added are Gemma7B, Mistral7B and a Unavailable Model named Mixtral which is more catered towards systems which can provide more resources for LLM generations.