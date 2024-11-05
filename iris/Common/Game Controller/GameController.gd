class_name GameController extends Node

@export var world: Node2D
@export var gui: Control

var current_world_scene: Node2D
var current_gui_scene: Control

func _ready() -> void:
	Global.game_controller = self
	current_gui_scene = $GUI/MainMenu

# Change the GUI Scene
func change_gui_scene(new_scene: String, delete: bool = true, keep_running: bool = false) -> void:
	if current_gui_scene != null:
		if delete:
			current_gui_scene.queue_free()
		elif keep_running:
			current_gui_scene.visible = false
		else:
			gui.remove_child(current_gui_scene)
	
	if new_scene != "":
		var new_gui_scene: Control = load(new_scene).instantiate()
		gui.add_child(new_gui_scene)
		current_gui_scene = new_gui_scene

# Change the World Scene
func change_world_scene(new_scene: String, delete: bool = true, keep_running: bool = false) -> void:
	if current_world_scene != null:
		if delete:
			current_world_scene.queue_free() # Removes Node
		elif keep_running:
			current_world_scene.visible = false # Node in Memory and Running
		else:
			world.remove_child(current_world_scene) # In Memory Not Running
	
	if new_scene != "":
		var new_world_scene: Node2D = load(new_scene).instantiate()
		world.add_child(new_world_scene)
		current_world_scene = new_world_scene
