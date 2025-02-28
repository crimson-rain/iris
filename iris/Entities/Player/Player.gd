#	FILENAME: Player.gd
#
#	Description
#   Interaction Area for Objects......
# TODO: Complete and Describe This..
#
#	NOTES
#   
#	AUTHOR: Rezwan Rahman (RAH22529097)
#	CREATED: 09/11/2024
#	MODIFIED: 09/11/2024

extends CharacterBody2D

@export var speed: float = 300.0
@export var acceleration: float = 10.0
@onready var player_animation_player: AnimationPlayer = $PlayerAnimationPlayer


func _physics_process(_delta: float) -> void:
	movement()


func movement() -> void:
	var direction: Vector2 = Input.get_vector("move_left", "move_right", "move_up", "move_down")
	
	if direction == Vector2.ZERO:
		velocity = Vector2.ZERO
	else:
		velocity = direction * speed
	
	update_animation(direction)
	move_and_slide()


func update_animation(direction: Vector2) -> void:
	if direction.x < 0: 
		player_animation_player.play("walk_left")
	elif direction.x > 0:
		player_animation_player.play("walk_right")
	elif direction.y < 0:
		player_animation_player.play("walk_up")
	elif direction.y > 0:
		player_animation_player.play("walk_down")
	else:
		player_animation_player.seek(0.35, true)
		player_animation_player.pause()
