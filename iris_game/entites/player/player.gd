extends CharacterBody2D

## Speed of the Character, When Moving
const SPEED: float = 145.0
## Acceleration of the Character, Increases the Speed with a Multiplier
const ACCELERATION: float = 15.0

## Animation Player
@onready var player_animation_player: AnimationPlayer = $AnimationPlayer

## Handles the Physics Process in the Game, Such as Movement Etc.
func _physics_process(_delta: float) -> void:
	movement()

## Handles the Movement of the Player Currently Manages 4-D Movement
func movement() -> void:
	if DialogueManager.is_dialogue_active or DialogueManager.chat_box_active:
		return
	
	var direction: Vector2 = Input.get_vector("move_left", "move_right", "move_up", "move_down")
	
	if direction == Vector2.ZERO:
		velocity = Vector2.ZERO
	else:
		velocity = direction * SPEED
	
	update_animation(direction)
	move_and_slide()

## Update the Animation Based on the Movement of the Player
func update_animation(direction: Vector2) -> void:
	if direction.x < 0: 
		player_animation_player.play("move_left")
	elif direction.x > 0:
		player_animation_player.play("move_right")
	elif direction.y < 0:
		player_animation_player.play("move_up")
	elif direction.y > 0:
		player_animation_player.play("move_down")
	else:
		player_animation_player.seek(0.35, true)
		player_animation_player.pause()
