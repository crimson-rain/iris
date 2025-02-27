extends MarginContainer

@onready var label: Label = $MarginContainer/Label
@onready var character_display_timer: Timer = $CharacterDisplayTimer

const MAX_WIDTH = 256

var text = ""
var letter_index = 0
