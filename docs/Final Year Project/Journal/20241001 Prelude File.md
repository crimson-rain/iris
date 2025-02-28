---
date: 2024-10-01
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/6550527ff47ceff06455d6764932486c571a7b53
---
Today, I setup the prelude file, this will contain common namespaces such and crates which I will be using. Since I will be implementing my own errors I will also have to implement the error crate inside prelude. 

The purpose of a prelude crate is to import multiple requirements in just one file, while also customizing specific preferences.

I have also started research on what packages I will need to handle the interaction with LLMs.

I have chosen [ollama-rs](https://crates.io/crates/ollama-rs) to help me with these interactions as the final package.