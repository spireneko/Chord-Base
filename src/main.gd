extends Control

@onready var content: PanelContainer = $VBoxContainer/Content

var songs_list_packaged := preload("res://src/scenes/ui/songs_list.tscn")

func _ready() -> void:
	var songs_list := songs_list_packaged.instantiate()
	content.add_child.call_deferred(songs_list)
