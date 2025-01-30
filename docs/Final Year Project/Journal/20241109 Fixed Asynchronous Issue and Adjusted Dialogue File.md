---
date: 2024-11-09
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/c884fc943302dba82f64c721b91c0006fe2d65ea
---
After making changes to the LLM file, the bug which blocked the main thread appeared again, this took me a couple of days to fix and ensure that it all worked. Previously mentioned I read through tokio documentation, and noticed that I needed to use channels to transfer and mutate data from one thread to another. This resolved the bug and ensured the main thread wouldn't be blocked. 

This time was also spent adjusting dialogue generation system. Ensuring that dialogue is properly created, and primarily refactoring code. 