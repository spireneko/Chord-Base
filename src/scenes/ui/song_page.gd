extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var input := "Я<Am> бегу по вы<E>жженной земле<Am>,\n" \
+"Гермошлем зах<Dm>лопнув н<G>а ходу<C>.    <E>\n" \
+"М<Am>ой \"Фантом\" стрел<C>ою белой на<D> распластанно<F>м крыле\n" \
+"С р<Am>евом набира<E>ет высоту<Am>.    <E>"
	print(input)

	var parser := SongWithChordsParser.create(input)
	print(parser.chords)
	print(parser.chords_positions)
	print(parser.song_text)

	print(parser.chords_positions[3].chord)
	print(parser.chords_positions[3].line)
	print(parser.chords_positions[3].offset)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
