# npc.gd |  28.04.2025
#
# General NPC Script for handling interactions between the player and NPCs

extends NPC

@onready var iris: Iris = $Iris
@onready var interaction_area: InteractionArea = $InteractionArea

var is_interacting: bool = false

func _ready() -> void:
	interaction_area.interact = Callable(self, "_on_interact")
	
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

func _handle_user_response(response: String) -> void:
	
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
