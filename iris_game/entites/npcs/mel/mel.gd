# mel.gd | 19.03.2025
# 
# NPC Script for Mel, can be exapanded so that it can be globally used by all NPCs, requires renaming.

extends NPC

@onready var iris: Iris = $Iris
@onready var interaction_area: InteractionArea = $InteractionArea

func _ready() -> void:
	interaction_area.interact = Callable(self, "_on_interact")
	DialogueManager.user_responded.connect(_handle_user_response)
	
func _on_interact(prompt: String) -> void:
	if DialogueManager.is_dialogue_active or DialogueManager.chat_box_active:
		return
	
	iris.generate_dialogue(prompt, self.get_npc_info())

func _on_iris_dialogue_generated(response: String) -> void:
	DialogueManager.start_dialogue(self.position, [response])

func _handle_user_response(response: String) -> void:
	iris.generate_dialogue(response, self.get_npc_info())
