---
date: 2024-09-30
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url:
---
# Error Handling and Packages
Today I have setup error handling using external libraries in rust. Primarily I have setup thiserror. Rust handles errors using return values. Rust uses a type system, specifically the Result keyword which has two types a `Result<T, E>`. The T represents the expected return type and the E represents the error type. Using this idiomatic approach allows us to avoid massive try except chains, thus improving readability of code, and when things don't go as they expect rust can handle the error. 

thiserror improves this system by introducing an error enum which stores all types of errors which can occur, as well as returning a message relevant to the error. 

Further reading and reference [thiserror documentaiton](https://docs.rs/thiserror/latest/thiserror/). 

I've also used anyhow another error handling library which allows for more descriptive and specific errors. Further reading and reference [anyhow documentation](https://docs.rs/anyhow/latest/anyhow/).