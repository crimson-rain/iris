# mel.gd | 19.03.2025
# 
# NPC script for Mel

extends NPC

@onready var iris: Iris = $Iris
@onready var interaction_area: InteractionArea = $InteractionArea

func _ready() -> void:
	interaction_area.interact = Callable(self, "_on_interact")
	DialogueManager.user_responded.connect(_handle_user_response)
	
func _on_interact(prompt: String) -> void:
	iris.generate_dialogue(prompt, self.get_npc())

# Currently, using an Array to Parse Response to Dialogue Manager, instead of just Parsing String Fix this Later
func _on_iris_dialogue_generated(response: String) -> void:
	DialogueManager.start_dialogue(self.position, [response])

func _handle_user_response(response: String) -> void:
	print("Response: ", response)
	iris.generate_dialogue(response, self.get_npc())
