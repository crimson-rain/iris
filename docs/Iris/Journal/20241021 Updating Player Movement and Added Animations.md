---
date: 2024-10-21
tags:
  - Documentation
  - FYP
cssclasses: 
commit-url: https://github.com/Crimson-Rain/iris/commit/d05236971e696ad10be4ba7845d3218673446f46
---
## Summary  
*Overhauled player movement to be more fluid, and added animations to movement.*

---
## Log
#### Updating the Player Movement
Updated the player movement and overhauled the fluidity of the movement as well as allowing the developer to adjust the speed and acceleration of the player. Separating the player movement and animation into different functions.

---

## Code Snippets
`Player.gd`
```ruby
func movement() -> void:

	var direction: Vector2 = Input.get_vector("move_left", "move_right", "move_up", "move_down")
	if direction == Vector2.ZERO:
		velocity = Vector2.ZERO
	else:
		velocity = direction * speed

	update_animation(direction)
	move_and_slide()
```