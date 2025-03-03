---
date: 2025-01-17
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/crimson-rain/iris/commit/e4d2a80596f50ae3aebbebea118187877423485e
---
I've implemented memory to my model, this allows the NPCs to have memory based on how recent or how many times the memory has been accessed. Currently using a HashMap to store a collection of Memory Structs. The Memory struct stores when the memory was created, how many times it has been accessed and the information of the memory itself.

I'll look to improve this at a later date.