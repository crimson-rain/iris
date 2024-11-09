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

@onready var text_box_scene: PackedScene = preload("res://Utilities/TextBox/TextBox.tscn")

var dialog_lines: Array[String] = []
var current_line_index: int = 0

var text_box: MarginContainer
var text_box_position: Vector2

var is_dialog_active: bool = false
var can_advance_line: bool = false

func start_dialog(position: Vector2, lines: Array[String]) -> void:
	if is_dialog_active:
		return
	
	dialog_lines = lines
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
