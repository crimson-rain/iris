#	FILENAME: DialogManager.gd
#
#	Description
#   Interaction Area for Objects
#
#	NOTES
#   
#	AUTHOR: Rezwan Rahman (RAH22529097)
#	CREATED: 09/11/2024
#	MODIFIED: 09/11/2024

extends Node

const text_box_scene: PackedScene = preload("res://Utilities/TextBox/TextBox.tscn")
const CHOICES_DIALOGUE: PackedScene = preload("res://Common/ChoicesDialogue/ChoicesDialogue.tscn")

var dialog_lines: Array[String] = []
var choices_arr: Array = []
var current_line_index: int = 0

var text_box: MarginContainer
var text_box_position: Vector2

var is_dialog_active: bool = false
var can_advance_line: bool = false

var current_npc: LLMCharacterBody2D

func start_dialog(position: Vector2, lines: Array[String], choices: Array, npc: LLMCharacterBody2D) -> void:
	if is_dialog_active:
		return
	
	current_npc = npc
	dialog_lines = lines
	choices_arr = choices
	text_box_position = position
	_show_text_box()
	
	is_dialog_active = true


func _show_text_box() -> void:
	text_box = text_box_scene.instantiate()
	get_tree().root.add_child(text_box)
	text_box.global_position = text_box_position
	
	text_box.connect("finished_displaying", Callable(self, "_on_text_box_finished_displaying"))
	
	text_box.display_text(dialog_lines[current_line_index])
	can_advance_line = false

func _on_text_box_finished_displaying() -> void:
	var choices_box: PanelContainer = CHOICES_DIALOGUE.instantiate()
	choices_box.global_position = Vector2(text_box_position.x + 100, text_box_position.y)
	get_tree().root.add_child(choices_box)
	choices_box.call("set", "choices", choices_arr)
	choices_box.connect("SELECTED", Callable(self, "_on_choices_dialogue_selected"))
	
	can_advance_line = true

func _unhandled_input(event: InputEvent) -> void:
	if (
		event.is_action_pressed("action_interact") &&
		is_dialog_active &&
		can_advance_line
	):
		text_box.queue_free()
		
		current_line_index += 1
		if current_line_index >= dialog_lines.size():
			is_dialog_active = false
			current_line_index = 0
			return
		
		_show_text_box()

func _on_choices_dialogue_selected(index: int) -> void:
	print("Choice Selected: %d" % index)
	
	if current_npc and current_npc.has_method("handle_interactions"):
		text_box.queue_free()
		current_npc.handle_interactions(choices_arr[index])
	else:
		print("Error: NPC does not have handle_interaction() method.")
	
	is_dialog_active = false
