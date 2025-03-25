# text_input.gd | 19.03.2025
# 
# 

extends Control

@onready var text_edit: TextEdit = $MarginContainer/HBoxContainer/TextEdit

signal generated_dialogue(response: String)
signal exit_signal()

func _ready() -> void:
	text_edit.focus_entered.connect(_on_text_edit_focus_entered)
	text_edit.focus_exited.connect(_on_text_edit_focus_exited)

func _input(event: InputEvent) -> void:
	if text_edit.has_focus():
		get_viewport().set_input_as_handled()

func _on_text_edit_focus_entered() -> void:
	# This will make the TextEdit capture all input
	set_process_input(false)
	get_tree().root.set_input_as_handled()

func _on_text_edit_focus_exited() -> void:
	set_process_input(true)

func _on_submit_button_pressed() -> void:
	emit_signal("generated_dialogue", text_edit.text)
	text_edit.text = ""

func _on_exit_button_pressed() -> void:
	emit_signal("exit_signal")
