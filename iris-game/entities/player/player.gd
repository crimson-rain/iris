## File: player.gd
## Created: YYYY-MM-DD
## Modified: YYYY-MM-DD
##
## Description
## This is the script for the main player.
## Manages the players movements and animation logic.
##
## Author: Rezwan Rahman (Crimson-Rain//RAH22529097)

extends CharacterBody2D

## Speed of the Character, When Moving
const SPEED = 300.0
## Acceleration of the Character, Increases the Speed with a Multiplier
const ACCELERATION = 10.0

## Animation Player
@onready var player_animation_player: AnimationPlayer = $PlayerAnimationPlayer

## Handles the Physics Process in the Game, Such as Movement Etc.
func _physics_process(_delta: float) -> void:
	movement()

## Handles the Movement of the Player Currently Manages 4-D Movement
func movement() -> void:
	var direction: Vector2 = Input.get_vector("move_left", "move_right", "move_up", "move_down")
	
	if direction == Vector2.ZERO:
		velocity = Vector2.ZERO
	else:
		velocity = direction * SPEED
	
	update_animation(direction)
	move_and_slide()

## Update the Animation Based on the Movement of the Player
func update_animation(direction: Vector2) -> void:
	match direction:
		Vector2(0, -1):
			print("Up")
		Vector2(0, 1):
			print("Down")
		Vector2(-1, 0):
			print("Left")
		Vector2(1, 0):
			print("Right")
		_:
			print("Stop")
