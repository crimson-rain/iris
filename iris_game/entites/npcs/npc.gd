# npc.gd |  28.04.2025
#
# General NPC Script for handling interactions between the player and NPCs

extends NPC

# STOP DISTANCE
@export var stop_distance: float = 75.0

@onready var iris: Iris = $Iris
@onready var interaction_area: InteractionArea = $InteractionArea
@onready var player: CharacterBody2D = get_tree().get_first_node_in_group("Player")
@onready var animation_player: AnimationPlayer = $AnimationPlayer

# Speed of the Character, When Moving
const SPEED: float = 145.0
# Acceleration of the Character, Increases the Speed with a Multiplier
const ACCELERATION: float = 15.0

var is_interacting: bool = false
var follow_player: bool = false

func _ready() -> void:
	interaction_area.interact = Callable(self, "_on_interact")

func _physics_process(_delta: float) -> void:
	if follow_player:
		start_follow_player()

func _on_interact(prompt: String) -> void:
	if DialogueManager.is_dialogue_active or DialogueManager.chat_box_active:
		return
	
	is_interacting = true
	
	if not DialogueManager.user_responded.is_connected(_handle_user_response):
		DialogueManager.user_responded.connect(_handle_user_response)
	
	iris.generate_dialogue(prompt, self.get_npc_info())

func _on_iris_dialogue_generated(response: String) -> void:
	if not is_interacting:
		return
	
	DialogueManager.start_dialogue(self.position, [response])

func start_follow_player() -> void:
	var current_distance: float = global_position.distance_to(player.global_position)
	
	if current_distance > stop_distance:
		var direction: Vector2 = global_position.direction_to(player.global_position)
		velocity = direction * SPEED
		update_animation(direction)
	else:
		velocity = Vector2.ZERO
		update_animation(Vector2.ZERO)
	
	move_and_slide()

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

func _handle_user_response(response: String) -> void:
	if response == "follow":
		follow_player = true
		return
	elif response == "stay":
		follow_player = false
		return
	
	if response.is_empty():
		_end_interaction()
		return
	
	iris.generate_dialogue(response, self.get_npc_info())

func _end_interaction() -> void:
	if not is_interacting:
		return
	
	is_interacting = false
	
	if DialogueManager.user_responded.is_connected(_handle_user_response):
		DialogueManager.user_responded.disconnect(_handle_user_response)
