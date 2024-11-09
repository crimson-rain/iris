#	FILENAME: InteractionArea.gd
#
#	Description
#   Manging MainMenu
# TODO: Complete and Describe This..
#
#	NOTES
#   
#	AUTHOR: Rezwan Rahman (RAH22529097)
#	CREATED: 09/11/2024
#	MODIFIED: 09/11/2024

extends Control

func _on_play_button_pressed() -> void:
	Global.game_controller.change_world_scene("res://World/Aether/Aether.tscn")
	Global.game_controller.change_gui_scene("")


func _on_setting_button_pressed() -> void:
	Global.game_controller.change_gui_scene("res://UI/Menus/Settings Menu/SettingsMenu.tscn")

# Quit the Game
func _on_quit_button_pressed() -> void:
	get_tree().quit()
