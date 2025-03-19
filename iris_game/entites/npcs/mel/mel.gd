extends NPC

@onready var iris: Iris = $Iris
@onready var interaction_area: InteractionArea = $InteractionArea

func _ready() -> void:
	interaction_area.interact = Callable(self, "_on_interact")

func _on_interact() -> void:
	iris.generate_dialogue("How is your day also what is the weather in Roehampton?", self.get_npc())
