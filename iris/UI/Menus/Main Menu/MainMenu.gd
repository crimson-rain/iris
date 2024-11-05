extends Control

func _on_play_button_pressed() -> void:
	Global.game_controller.change_world_scene("res://World/Aether/Aether.tscn")
	Global.game_controller.change_gui_scene("")


func _on_setting_button_pressed() -> void:
	Global.game_controller.change_gui_scene("res://UI/Menus/Settings Menu/SettingsMenu.tscn")

# Quit the Game
func _on_quit_button_pressed() -> void:
	get_tree().quit()
