# mel.gd | 19.03.2025
# 
# NPC script for Mel

extends NPC

@onready var iris: Iris = $Iris
@onready var interaction_area: InteractionArea = $InteractionArea

func _ready() -> void:
	interaction_area.interact = Callable(self, "_on_interact")

func _on_interact() -> void:
	iris.generate_dialogue("How is your day also what is the weather in Roehampton?", self.get_npc())

# Currently, using an Array to Parse Response to Dialogue Manager, instead of just Parsing String Fix this Later
func _on_iris_dialogue_generated(response: String) -> void:
	DialogueManager.start_dialogue(self.position, [response])
