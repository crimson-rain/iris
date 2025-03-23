# interaction_manager.gd | 19.03.2025
#
# DESCRIPTION
# The interaction manager is responsible for allowing two objects to interact,
# as well as organize multiple objects which can interact by prioritizing the one closest to the
# player.

extends Node2D

@onready var player: CharacterBody2D = get_tree().get_first_node_in_group("Player")
@onready var label: Label = $Label

const base_text: String = "[E] to "

var active_areas: Array[InteractionArea] = []
var can_interact: bool = true

func register_area(area: InteractionArea) -> void:
	active_areas.push_back(area)

func unregister_area(area: InteractionArea) -> void:
	var index: int = active_areas.find(area)
	
	if index != -1:
		active_areas.remove_at(index)

func _process(_delta: float) -> void:
	if active_areas.size() > 0 && can_interact:
		active_areas.sort_custom(_sort_by_nearest)
		label.text = base_text + active_areas[0].action_name
		label.global_position = active_areas[0].global_position
		label.global_position.y += 16
		label.global_position.x -= label.size.x / 3.3
		label.show()
	else:
		label.hide()

func _sort_by_nearest(area_one: InteractionArea, area_two: InteractionArea) -> bool:
	if player == null or area_one == null or area_two == null:
		return false
		
	var area_one_dist: float = player.global_position.distance_to(area_one.global_position)
	var area_two_dist: float = player.global_position.distance_to(area_two.global_position)
	return area_one_dist < area_two_dist

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("interact") && can_interact:
		if active_areas.size() > 0:
			can_interact = false
			label.hide()
			
			await active_areas[0].interact.call("Hello")
			
			can_interact = true
