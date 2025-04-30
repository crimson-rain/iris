extends CharacterBody2D

# Speed of the Character, When Moving
const SPEED: float = 145.0
# Acceleration of the Character, Increases the Speed with a Multiplier
const ACCELERATION: float = 15.0

# STOP DISTANCE
@export var stop_distance: float = 100.0

@export var health: int = 250
@export var damage: int = 100

# Animation Player
@onready var animation_player: AnimationPlayer = $AnimationPlayer

# Player
@onready var player: CharacterBody2D = get_tree().get_first_node_in_group("Player")

# Handles the Physics Process in the Game, Such as Movement Etc.
func _physics_process(_delta: float) -> void:
	movement()

# Handles the Movement of the Player Currently Manages 4-D Movement
func movement() -> void:
	var current_distance: float = global_position.distance_to(player.global_position)
	
	if current_distance > stop_distance:
		var direction: Vector2 = global_position.direction_to(player.global_position)
		velocity = direction * SPEED
		update_animation(direction)
	else:
		velocity = Vector2.ZERO
		update_animation(Vector2.ZERO)
	
	move_and_slide()

# Update the Animation Based on the Movement of the Player
func update_animation(direction: Vector2) -> void:
	if direction.x < 0: 
		animation_player.play("move_left")
	elif direction.x > 0:
		animation_player.play("move_right")
	elif direction.y < 0:
		animation_player.play("move_up")
	elif direction.y > 0:
		animation_player.play("move_down")
	else:
		animation_player.seek(0.35, true)
		animation_player.pause()
