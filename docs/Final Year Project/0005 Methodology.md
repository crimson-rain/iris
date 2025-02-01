---
date-created: 2025-01-13
date-modified: 
tags:
  - Documentation
  - FYP
cssclasses:
  - center-images
---
This section outlines all the methods and techniques used to design, implement and evaluate the solution for emergent dialogues and quest generations. This methodology ensures reproducibility and clarity in the development process.
## Design and Methodology
This research is experimental as well as applied. There has been proof that this system can be applied and implemented. 

### Current System architecture

![[System Architecture Diagram]]
<p style="text-align: center">Fig 1. High Level System Architecture</p>


- **LLM Module** - Responsible for natural language understanding and generation.
- **RAG Systems** - Stores knowledge retrieval for dynamic and context-aware responses.
- **NPC Memory** - Implements
	- **Short Term Memory** - Tracks recent interactions to maintain conversational coherence. 
	- **Long Term Memory** - Stores historical data for consistent NPC behaviour and personalization.


This is the current architecture of the LLM System. Currently working on the implementation of the LLM + RAG System to allow setup a long-term memory for NPCs and short-term memory. This is a WIP diagram and will be heavily updated in the future.

### Iterative Design

![[Iterative Design Cycle.png]]


<p style="text-align: center">Fig 2. Dual Iterative Design for Rapid Prototyping and Developing Final Prototype</p>

For Milestone 2, the focus will be on rapid development and prototyping, this phase prioritizes on achieving a functional, integrated solution with minimal testing on the features. The main goal of Milestone 2 is to ensure that the experiment is feasible. Once the experiment proves successful and the solution is verified to work as intended, the next phase will involve refactoring the codebase to align with industry-standards such as implementing design patterns and best practices.

The project will implement a dual iterative design approach
1. Rapid Prototype Development Phase (Milestone 2):
	- Rapidly Develop and Implement Features
	- Functionality over optimizations.
	- Conduct minimal feature testing to establish feasibility
2. Refinement Phase
	- Refactor the codebase to adhere to industry standards, including best practices and design patterns in rust and Godot.
	- Optimize system performance based on feedback and testing insights.

### Model Fine Tuning
After picking a model, the next-phase is to fine tune the model such that it will generate the expected response. The model will be trained to generate correct formatted responses, if it fails to abide by these strict rules it will be asked to generate until it is correct.

## Testing and Evaluation

### Rust Test Cases
Test cases found in rust will primarily be unit test to ensure that each individual component works as intended. Testing various things such as returns of functions, making sure that correct inputs are being parsed.

### GDUnit Test
GDUnit test will be used to test individual Godot scripts, to validate their correctness. Some examples is to test and verify player movement scripts. 

GDUnit test also provides the ability to do integration testing this will validate the interaction between multiple scripts or scenes. 

Integration test will be hosted on GDUnit as we can not test the integration test on rust as its a library.

Some Examples of Tests 
- Scene Validation
- Behaviour Testing
- Edge Case Testing
- Regression Testing
- Performance Testing
- Mocking and Simulating Events

### GitHub Workflow
GitHub is utilized to automate the testing process, including,
CI/CD Pipelines, which automatically run tests on code commits. Issue tracking to monitor and resolve bugs or feature requests, as well as milestone progression.

## Project Management
### GitHub Projects
Using GitHub Projects to manage various requirements, difficulty of implementation.
I will also be using GitHub Milestones to manage different milestones. Will also be using the GitHub issues to create requirements which need to be finished as well as bugs that need to be patched.


## Technologies and Processes
### Godot
Godot serves as the foundation and core of the project, functioning as the primary game engine. It will have game specific code and objects such as levels or scenes, characters and various game components, providing a unified environment for designing and testing the interactive game framework. Godot was selected because of its lightweight, open-source and ease of use nature. As well as its versatility and extensive support for 2D game development. 
### GDScript
GDScript will be the primary language for game specific logic, used to implement game logic within the Godot Engine. This scripting language was designed specifically for Godot, and has simple syntax similar to python, making the game development time quick.
### Ollama
Ollama is responsible for managing the LLM which will be integrated to the game. We will use Ollama's inbuilt API. It will be responsible for handling and responding to various requests. This will be the primary engine for the generating emergent and immersive dialogue and quests for the player to experience. 
### Git
Git will be used for its version control, enabling efficient management of code changes. GitFlow branching model will be used to separate features, bug fixes and releases for the project further enchaining its maintainability. 
### GitHub 
GitHub will host the repository of the project, using LFS GitHub will store images in the LFS server. GitHub will also help with project management, issue tracking and ensuring that the software is safely stored inside another medium.

#### Notes
This methodology is incomplete and is still a work in progress.