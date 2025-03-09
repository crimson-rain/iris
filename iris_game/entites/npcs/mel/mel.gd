extends NPC

@onready var iris: Iris = $Iris

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("interact"):
		iris.generate_dialogue()
