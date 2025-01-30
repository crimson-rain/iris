---
date: 2024-11-10
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/a9e3b18894855f4a23f4f2340cb4698a6cb32276
---
Started research on smart objects and how I could make use of them, This idea was from my supervisor, he suggested using smart objects which would store information and descriptions on the objects themselves.

I believe this would be a good idea to setup NPC interactions to items and objects.

The stuff I understood about smart object is that its a object which has information about itself stored within itself, meaning that all the logic needed to interact with the object is stored within itself. 

An example which I have come up with is, lets say you have a book object. Following the Smart Object framework, the book has information about how it can be used within itself. For example we can read the book by opening it, we can store the book somewhere, maybe we can pick it up, close the book after we finish reading it. This is the general of smart objects.

I believe this would be useful as it would allow us to store and describe smart objects.

Some ideas I have for smart objects is maybe having a interaction enum, which gives strict instructions on actions the player or NPC can take, and if the action isn't valid in the enum we can prompt this to the LLM or player.