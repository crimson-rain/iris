# interaction_area.gd | 19.03.2025
#
# Interaction area allows for players to interact with certain entites.
# This section handles the registering and de-registering of CharacterBody2D entering the area.

extends Area2D
class_name InteractionArea

@export var action_name: String = "Talk"

var interact: Callable = func() -> void:
	pass

func _ready() -> void:
	body_entered.connect(_on_body_entered)
	body_exited.connect(_on_body_exited)

func _on_body_entered(_body: CharacterBody2D) -> void:
	InteractionManager.register_area(self)

func _on_body_exited(_body: CharacterBody2D) -> void:
	InteractionManager.unregister_area(self)
