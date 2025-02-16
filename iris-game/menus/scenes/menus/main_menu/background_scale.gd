extends AnimatedSprite2D

var base_resolution = Vector2(800, 450)

func _ready() -> void:
	get_tree().root.connect("size_changed", _on_viewport_size_changed)
	update_scale()
	
func update_scale():
	var viewport_size = get_viewport().size
	var scale_factor = max(viewport_size.x / base_resolution.x, viewport_size.y / base_resolution.y)
	self.scale = Vector2(scale_factor, scale_factor)

func _on_viewport_size_changed():
	update_scale()
