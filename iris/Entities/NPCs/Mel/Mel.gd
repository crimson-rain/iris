extends LLMCharacterBody2D

@onready var interaction_area: InteractionArea = $InteractionArea

func _on_generated_dialogue(response: String) -> void:
	DialogManager.start_dialog(global_position, [response])
