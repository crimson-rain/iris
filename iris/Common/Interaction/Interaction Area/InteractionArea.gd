#	FILENAME: InteractionArea.gd
#
#	Description
#   Interaction Area for Objects......
# TODO: Complete and Describe This..
#
#	NOTES
#   
#	AUTHOR: Rezwan Rahman (RAH22529097)
#	CREATED: 09/11/2024
#	MODIFIED: 09/11/2024

class_name InteractionArea extends Area2D

@export var action_name: String = "interact"

func _on_body_entered(_body: Node2D) -> void:
	InteractionManager.register_area(self)

func _on_body_exited(_body: Node2D) -> void:
	InteractionManager.unregister_area(self)
