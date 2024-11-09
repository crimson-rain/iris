#	FILENAME: InteractionArea.gd
#
#	Description
#   Manging MainMenu
# 
#
#	NOTES
#   TODO: Complete and Describe This..
#
#	AUTHOR: Rezwan Rahman (RAH22529097)
#	CREATED: 09/11/2024
#	MODIFIED: 09/11/2024

extends Control

# Start Games
func _on_play_button_pressed() -> void:
	Global.game_controller.change_world_scene("res://World/Aether/Aether.tscn")
	Global.game_controller.change_gui_scene("")

# Go to Settings Menu
func _on_setting_button_pressed() -> void:
	Global.game_controller.change_gui_scene("res://UI/Menus/Settings Menu/SettingsMenu.tscn")

# Quit the Game
func _on_quit_button_pressed() -> void:
	get_tree().quit()
