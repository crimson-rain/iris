extends LLMCharacter

@export var npc_res: Resource

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	if npc_res and npc_res.texture:
		var sprite = $Sprite2D  # Assuming there's a Sprite2D node as a child
		sprite.texture = npc_res.texture
	else:
		print("NPC resource or texture not found!")

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass
