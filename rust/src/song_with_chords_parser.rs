use std::collections::HashSet;

use godot::prelude::*;
use logos::Logos;

#[derive(GodotClass)]
#[class(no_init)]
struct SongWithChordsParser {
    chords: PackedStringArray,
    song_text: GString,
}

#[godot_api]
impl SongWithChordsParser {
    // #[func]
    // fn create(input: String) -> Gd<Self> {
    //     let mut lexer = Token::lexer(&input);
    //     let mut chords_set = HashSet::new();
    //     let mut song_text = GString::new();

    //     while let Some(token) = lexer.next() {
    //         match token {
    //             Err(_) => break,
    //             Ok(token) => match token {
    //                 Token::Chord(chord) => {
    //                     chords_set.insert(chord);
    //                 }
    //             },
    //         }
    //     }

    //     Gd::from_object(Self {
    //         chords: PackedStringArray::from(chords_set.into()),
    //         song_text,
    //     })
    // }
}

// Я<Am> бегу по вы<E>жженной земле<Am>,
// Гермошлем зах<Dm>лопнув н<G>а ходу<C>.    <E>
// М<Am>ой "Фантом" стрел<C>ою белой на<D> распластанно<F>м крыле
// С р<Am>евом набира<E>ет высоту<Am>.    <E>

#[derive(Logos)]
enum Token {
    /// Looking for chords inside <> brackets.
    /// Brakets will be excluded from token.
    #[regex(r"<[^>]+>", |lex| {
        let slice = lex.slice();
        let slice = &slice[1..slice.len()-1];
        GString::from(slice)
    })]
    Chord(GString),

    /// Looking for any text, that is not chord.
    #[regex(r"[^<]+", |lex| {
        let slice = lex.slice();
        GString::from(slice)
    })]
    Text(GString),
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use godot::{builtin::GString, meta::ToGodot};

    use super::SongWithChordsParser;

    #[test]
    fn it_works() {
        let input = r#"Я<Am> бегу по вы<E>жженной земле<Am>,
Гермошлем зах<Dm>лопнув н<G>а ходу<C>.    <E>
М<Am>ой "Фантом" стрел<C>ою белой на<D> распластанно<F>м крыле
С р<Am>евом набира<E>ет высоту<Am>.    <E>"#;

        let song_text = GString::from(
            r#"Я бегу по выжженной земле,
Гермошлем захлопнув на ходу.
Мой "Фантом" стрелою белой на распластанном крыле
С ревом набирает высоту."#,
        );
        let chords: HashSet<GString> = vec!["Am", "E", "C", "Dm", "G", "D"]
            .into_iter()
            .map(|chord| chord.into())
            .collect();

        // let parser = SongWithChordsParser::create(input.to_string());

        // assert_eq!(parser.bind().song_text, song_text);
        // assert_eq!(HashSet::from_iter(parser.bind().chords.to_vec()), chords)
    }
}
