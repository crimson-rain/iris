# dialogue_manager.gd | 19.03.2025
#
# DESCRIPTION
# Dialogue Managers handles the displaying and rendering of dialogue.
# This is a universal function and can be used to display either pre-written or dynamically generated dialogues.
# It will scale correctly, and will be display above an entities global position.

extends Node

signal user_responded(response: String)

@onready var text_box_scene: PackedScene = preload("res://systems/dialogue/text_box.tscn")
@onready var chat_box_scene: PackedScene = preload("res://systems/text_input/text_input.tscn")

var dialogue_line: Array[String] = []
var current_line_index: int = 0

var text_box: Control
var text_box_position: Vector2

var chat_box: Control

var is_dialogue_active: bool = false
var can_advance_line: bool = false
var chat_box_active: bool = false  # Prevent multiple chat boxes

func start_dialogue(position: Vector2, lines: Array[String]) -> void:
	print("Completed Dialogue Generation [Dialogue]: ", lines)
	
	if is_dialogue_active or chat_box_active:  # Prevent restarting if chat is active
		return
	
	dialogue_line = lines
	text_box_position = position
	current_line_index = 0  # Reset to avoid repeats
	is_dialogue_active = true
	_show_text_box()

func _show_text_box() -> void:
	if current_line_index >= dialogue_line.size():
		_end_dialogue()
		return
	
	print("Adding text box to the scene tree")
	text_box = text_box_scene.instantiate()
	text_box.finished_displaying.connect(_on_text_box_finished_displaying)
	get_tree().root.add_child(text_box)
	text_box.global_position = text_box_position
	
	print("Sending text to text box: ", dialogue_line[current_line_index])
	text_box.display_text(dialogue_line[current_line_index])
	
	can_advance_line = false

func _on_text_box_finished_displaying() -> void:
	can_advance_line = true

func _unhandled_input(event: InputEvent) -> void:
	if event.is_action_pressed("continue") and is_dialogue_active and can_advance_line:
		text_box.queue_free()
		
		current_line_index += 1
		if current_line_index >= dialogue_line.size():
			_end_dialogue()
		else:
			_show_text_box()

func _end_dialogue() -> void:
	is_dialogue_active = false
	can_advance_line = false
	
	if text_box and is_instance_valid(text_box):
		text_box.queue_free()
		text_box = null
	
	_show_chat_box()  # Display chat box when dialogue ends

func _show_chat_box() -> void:
	# Prevents Multiple Chats
	if chat_box_active:
		return 
	
	chat_box_active = true
	chat_box = chat_box_scene.instantiate()
	get_tree().root.add_child(chat_box)
	chat_box.generated_dialogue.connect(_process_user_response)
	chat_box.exit_signal.connect(_process_exit_signal)
	chat_box.global_position = text_box_position + Vector2(50, 50)

func _process_user_response(response: String) -> void:
	chat_box_active = false
	chat_box.queue_free()
	user_responded.emit(response)

func _process_exit_signal() -> void:
	chat_box_active = false
	chat_box.queue_free()
