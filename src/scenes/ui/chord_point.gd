extends Control

@onready var empty: Label = $Empty
@onready var chord: Label = $Chord

func _ready() -> void:
	chord.position.y -= empty.get_line_height(1)
