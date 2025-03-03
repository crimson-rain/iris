---
date: 2025-01-19
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/crimson-rain/iris/commit/5f30780270b221dd0e67d76668cf7798273ef0eb
---
Fixed Doc Tests, the main cause of these errors was because the library wasn't being added when running the actual tests. Document tests allows for rust to generate API documents using cargo doc.