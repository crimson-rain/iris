# text_input.gd | 19.03.2025
# 
# 

extends Control

@onready var text_edit: TextEdit = $MarginContainer/HBoxContainer/TextEdit

signal generated_dialogue(response: String)
signal exit_signal()

const EXTRA_VERTICAL_PADDING: int = 8

func _ready() -> void:
	text_edit.custom_minimum_size = Vector2(148.0, 25.0)
	text_edit.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	text_edit.size_flags_vertical = Control.SIZE_SHRINK_CENTER

func _on_submit_button_pressed() -> void:
	emit_signal("generated_dialogue", text_edit.text)
	text_edit.text = ""

func _on_exit_button_pressed() -> void:
	emit_signal("exit_signal")
