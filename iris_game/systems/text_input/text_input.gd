# text_input.gd | 19.03.2025
# 
# 

extends Control

@onready var text_edit: TextEdit = $MarginContainer/HBoxContainer/TextEdit

signal generated_dialogue(response: String)
signal exit_signal()

func _ready() -> void:
	pass

func _on_submit_button_pressed() -> void:
	emit_signal("generated_dialogue", text_edit.text)
	text_edit.text = ""

func _on_exit_button_pressed() -> void:
	emit_signal("exit_signal")
