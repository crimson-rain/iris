extends Area2D
class_name InteractionArea

@export var action_name: String = "interact"

var interact: Callable = func() -> void:
	pass

func _ready() -> void:
	body_entered.connect(_on_body_entered)
	body_exited.connect(_on_body_exited)

func _on_body_entered(_body: CharacterBody2D) -> void:
	InteractionManager.register_area(self)
	print("Entered")

func _on_body_exited(_body: CharacterBody2D) -> void:
	InteractionManager.unregister_area(self)
	print("Exited")
