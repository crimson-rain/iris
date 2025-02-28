---
date: 2024-11-08
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/115ca33ee24e5ccd5027a76607aa2049f165b8db
---
Today I refactored the LLM file to make it more orderly, this involved separating the system messages. 

Using an Enum to manage the various models, as well as a factory method to create a new model.

I've also re-implemented the generation functions, such that it would be organized in the LLM file.