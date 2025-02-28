extends PanelContainer

signal SELECTED(index: int)

@onready var choices_list: VBoxContainer = $MarginContainer/Choices
@onready var choice_button: Button = $MarginContainer/Choices/ChoiceButton

var choices: Array:
	set(value):
		_choices = value
		create_choices_box()

var _choices: Array = []

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	# Ensure the initial choice button is hidden since we dynamically create new boxes.
	choice_button.visible = false

func create_choices_box() -> void:
	# Create a new VBoxContainer for choices.
	var new_choices_box: VBoxContainer = VBoxContainer.new()
	new_choices_box.name = "ChoicesBox"
	add_child(new_choices_box)
	
	# Generate buttons for each choice.
	for choice_index: int in range(_choices.size()):
		var new_button: Button = choice_button.duplicate()
		new_button.visible = true
		new_button.text = _choices[choice_index]
		new_button.pressed.connect(_on_choice_selected.bind(choice_index, new_choices_box))
		new_choices_box.add_child(new_button)

func _on_choice_selected(choice_index: int, choices_box: VBoxContainer) -> void:
	# Remove the choices box and emit the selected signal.
	choices_box.queue_free()
	emit_signal("SELECTED", choice_index)
	self.queue_free()
