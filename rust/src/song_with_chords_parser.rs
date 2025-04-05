use std::collections::HashSet;

use godot::prelude::*;
use logos::Logos;

#[derive(GodotClass)]
#[class(no_init)]
struct SongWithChordsParser {
    #[var(get)]
    chords: PackedStringArray,
    #[var(get)]
    chords_positions: Array<Gd<ChordPosition>>,
    #[var(get)]
    song_text: PackedStringArray,
}

#[godot_api]
impl SongWithChordsParser {
    #[func]
    fn create(input: String) -> Gd<Self> {
        let mut lexer = Token::lexer(&input);
        let mut chords_set = HashSet::new();
        let mut song_text = PackedStringArray::new();
        let mut line_text = String::new();
        let mut chords_positions = Array::new();

        let mut line_num = 0;
        let mut line_offset = 0;

        while let Some(token) = lexer.next() {
            match token {
                Err(_) => break,
                Ok(token) => match token {
                    Token::Chord(chord) => {
                        chords_set.insert(chord.clone());
                        let position = ChordPosition {
                            chord: chord.into(),
                            line: line_num,
                            offset: line_offset,
                        };
                        chords_positions.push(&Gd::from_object(position));
                    }
                    Token::Text(text) => {
                        line_offset += text.chars().count() as u32;
                        line_text.push_str(&text);
                    }
                    Token::Newline => {
                        line_num += 1;
                        line_offset = 0;
                        song_text.push(&GString::from(line_text.clone()));
                        line_text.clear();
                    }
                },
            }
        }

        if !line_text.is_empty() {
            song_text.push(&GString::from(line_text));
        }

        let chords_list: Vec<GString> = chords_set.into_iter().map(|chord| chord.into()).collect();

        Gd::from_object(Self {
            chords: PackedStringArray::from(chords_list),
            chords_positions,
            song_text: song_text.into(),
        })
    }
}

#[derive(Logos)]
enum Token {
    /// Looking for chords inside <> brackets.
    /// Brackets will be excluded from token.
    #[regex(r"<[^>]+>", |lex| {
        let slice = lex.slice();
        slice[1..slice.len()-1].to_string()
    })]
    Chord(String),

    /// Looking for newline separators.
    #[regex(r"\n")]
    Newline,

    /// Looking for any text, that is not chord.
    #[regex(r"[^[<\n]]+", |lex| lex.slice().to_string())]
    Text(String),
}

#[derive(GodotClass)]
#[class(no_init)]
struct ChordPosition {
    #[var(get)]
    chord: GString,
    #[var(get)]
    line: u32,
    #[var(get)]
    offset: u32,
}
