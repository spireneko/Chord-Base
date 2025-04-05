class_name LineWithChords
extends Control

# Текст для отрисовки
@export var line1: String = "Это очень длинная строка для тестирования переноса"
@export var line2: String = "Еще одна длинная строка, которая должна быть синхронизирована"

# Используемый шрифт (желательно моноширинный)
@export var font: Font
var font_size := get_theme_font_size("normal_font_size")

func _ready() -> void:
	# Если шрифт не задан в инспекторе, попробуем взять из темы
	if not font:
		font = ThemeDB.get_default_theme().default_font
	queue_redraw()

func _notification(what: int) -> void:
	if what == NOTIFICATION_RESIZED:
		queue_redraw()

func _draw() -> void:
	# Получаем массивы сегментов для первой и второй строк
	var result := wrap_text(line1, line2)
	var segments1 = result[0]
	var segments2 = result[1]
	
	# Расчитываем высоту строки
	var line_height := font.get_height()
	var y := 0.0
	# Рисуем пары строк последовательно
	for i in range(segments1.size()):
		draw_string(font, Vector2(0, y + line_height), segments1[i])
		draw_string(font, Vector2(0, y + line_height * 2), segments2[i])
		y += line_height * 2

# Функция, реализующая синхронный перенос текста по пробелам
func wrap_text(text1: String, text2: String) -> Array:
	var segments1: Array[String] = []
	var segments2: Array[String] = []
	
	# Для выравнивания символов приводим строки к одинаковой длине, дополняя пробелами (если это необходимо)
	var max_len: int = max(text1.length(), text2.length())
	if text1.length() < max_len:
		text1 += " ".repeat(max_len - text1.length())
	if text2.length() < max_len:
		text2 += " ".repeat(max_len - text2.length())
	
	var start := 0
	# Доступная ширина элемента
	var available_width := size.x
	# Получаем ширину одного символа (для моноширинного шрифта)
	var char_width := font.get_char_size(65, font_size).x
	var max_chars := int(available_width / char_width)
	
	while start < max_len:
		var end: int = min(start + max_chars, max_len)
		var break_index := end
		
		# Если строка не полностью уместилась, ищем последний общий пробел для переноса
		if end < max_len:
			var found_space := false
			for i in range(end, start, -1):
				if text1[i - 1] == " " and text2[i - 1] == " ":
					break_index = i
					found_space = true
					break
			# Если пробела нет, разрываем строго по количеству символов
			if not found_space:
				break_index = end
		# Извлекаем сегменты и убираем лишние пробелы по краям
		segments1.append(text1.substr(start, break_index - start).strip_edges())
		segments2.append(text2.substr(start, break_index - start).strip_edges())
		start = break_index
		# Пропускаем начальные пробелы следующего сегмента
		while start < max_len and text1[start] == " ":
			start += 1
	return [segments1, segments2]
