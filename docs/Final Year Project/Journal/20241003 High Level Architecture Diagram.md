---
date: 2024-10-03
tags:
  - Documentation
  - FYP
cssclasses:
  - center-images
commit-url: https://github.com/Crimson-Rain/iris/commit/ec07d9712bcae5f768c4155f5690396fd93b46c4
---
![[High Level Architecture Diagram.png]]

Today I have designed a High Level Architecture Diagram, to better understand how I should organize my code. From this diagram I understand that I need two main components, one for the game and another for the API interacting with the game which generates the dialogue. I believe there's a better way to design this and will work on a better implementation and design for now this will be used to rapidly develop a simple scaffold.

This diagram shows how game data will flow from the game to the API which will generate and return dialogue/quest from the LLML. 