# IRIS

<p align="center">
  <img src="https://i.imgur.com/Awqaduo.gif" alt="Animated Banner" /> <br>
  <em>Summer Wars (2009)</em>
</p>

**Intelligent Response and Interaction System (IRIS)** is a system used to generate emergent, dynamic, and contextually aware interactions for NPCs using LLMs. 
It aims to move beyond static, pre-written lines by allowing NPCs to respond dynamically and adapt their interactions with the player based on player choices. 
world events, and character personalities.

## Getting Started
Before you start and run the project, you will need to install the dependencies for this project.

### Dependencies
- Godot
- Ollama
- Rust
- Docker

### Set up Instructions
1. First, pull the Llama3.1 model from Ollama by running `ollama pull granite3.3`
2. Next, run `docker-compose up -d`, this will run the qdrant vector database, to store documents for the game world.
3. Run `cargo test`, this will fail one test, but set up the RAG database.
4. After this is done, open up the Godot game, which is found in iris_godot.
5. You can now interact with the NPCs and create your own NPCs, which can communicate with the RAG system for world information.

## Additional Information
Visit the Wiki...

## Acknowledgement
N/A...
