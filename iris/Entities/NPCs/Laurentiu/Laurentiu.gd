#	FILENAME: Aiden.gd
#
#	Description
#   Interaction Area for Objects
#
#	NOTES
#   
#	AUTHOR: Rezwan Rahman (RAH22529097)
#	CREATED: 11/11/2024
#	MODIFIED: 11/11/2024

extends LLMCharacterBody2D

@onready var interaction_area: InteractionArea = $InteractionArea

func _on_generated_dialogue(response: String) -> void:
	DialogManager.start_dialog(global_position, [response])
