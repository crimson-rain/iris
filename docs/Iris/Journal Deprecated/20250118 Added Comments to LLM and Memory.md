---
date: 2025-01-18
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/crimson-rain/iris/commit/81b84bfa19a17b4754e6ef00fba02c3b22d89c3d#diff-4753def9ca76e19d2efbf1a722efcf0bd266a252ac2f61d0919109051049f7cd
---
Today I started commenting LLM and Memory files completely and ensuring that extensive tests are created and cover all cases and edge cases. 

The commenting which I'm currently doing is following best practices found in many rust repositories such as tokio, wgpu and the rust standard library as well as my own design choices.

Since this is theoretical work I believe its important to store information of how its implemented. 

I've been reading Pragmatic Programmer, and it says that information should not be duplicated. For example, when commenting about memory, the only place which store information about how the memory works should be inside the memory module.

This principle is also known as DRY or Don't Repeat Yourself.