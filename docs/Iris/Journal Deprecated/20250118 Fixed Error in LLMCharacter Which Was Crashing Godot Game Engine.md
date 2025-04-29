---
date: 2025-01-18
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/crimson-rain/iris/commit/f45a8b9194da145cad9e5fcd7a339b1cd813702c
---
There was a bug which took my a while to problem solve. The problem was caused because I was using process, inside both my Godot code for my LLMCharacter and inside my rust library. This caused the process function inside my rust library get overwritten by Godot, which caused created an issue where the process function wasn't able to receive the generated dialogue from the other thread. 