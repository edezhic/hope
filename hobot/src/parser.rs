use crate::{Algebra::*, Control::*, Function::*, Preposition::*, Relation::*, Selector::*, *};
use regex::Regex as R;
use std::iter::Peekable;
use unicode_segmentation::UWordBoundIndices;

pub struct Parser<'a> {
    iter: Peekable<UWordBoundIndices<'a>>,
    pub peek: Option<(usize, &'a str)>,
}
impl<'a> Parser<'a> {
    pub fn convert(s: &'a str) -> Result<Vec<(usize, Token)>> {
        let text = Text::from_str(s);
        let iter = text.split_by_word_bounds().peekable();
        let mut parser = Parser { iter, peek: None };
        parser.update_peek();
        let mut vec = vec![];
        while let Some((index, piece)) = parser.peek {
            if let Some(value) = parser.match_value(piece)? {
                vec.push((index, value));
            } else if let Some(keyword) = Token::matches(piece) {
                vec.push((index, keyword));
                parser.next();
            } else if let Some(reference) = parser.match_reference(piece) {
                vec.push((index, reference));
            } else {
                return Err(Parsing(format!(
                    r#"I don't know how to translate '{:?}'"#,
                    piece
                )));
            }
        }
        Ok(vec)
    }
    fn update_peek(&mut self) {
        self.skip();
        if let Some(piece) = self.iter.peek() {
            self.peek = Some(*piece)
        } else {
            self.peek = None
        }
    }

    fn next(&mut self) -> Option<(usize, &'a str)> {
        self.iter.next();
        self.update_peek();
        self.peek
    }

    fn collect_text(&mut self) -> Text {
        self.iter.next();
        let mut text = Text::empty();
        while let Some((_, piece)) = self.iter.peek() {
            if TEXT.is_match(piece) {
                self.iter.next();
                break;
            } else {
                text.add(piece);
                self.iter.next();
            }
        }
        self.update_peek();
        text
    }

    fn collect_literal(&mut self) -> Text {
        self.iter.next();
        let mut text = Text::empty();
        while let Some((_, piece)) = self.iter.peek() {
            if LITERAL_END.is_match(piece) {
                break;
            } else {
                text.add(piece);
                self.iter.next();
            }
        }
        self.update_peek();
        text
    }

    fn skip(&mut self) {
        while let Some((_, piece)) = self.iter.peek() {
            if SKIP.is_match(piece) {
                self.iter.next();
            } else {
                break;
            }
        }
    }

    fn match_value(&mut self, piece: &str) -> Result<Option<Token>> {
        let value = match piece {
            piece if NUMBER.is_match(piece) => {
                let num = piece.replacen(",", ".", 1);
                self.next();
                Value::Num(Number::from_string(num)?)
            }
            piece if TEXT.is_match(piece) => Value::Txt(self.collect_text()),
            "{" => {
                self.next();
                Value::new_struct()
            }
            "[" => {
                self.next();
                Value::new_list()
            }
            "@" => Value::I(Id::from_text(self.collect_literal())?),
            "&" => Value::Sl(Seal::from_text(self.collect_literal())?),
            "~" => Value::Dt(Datetime::from_text(self.collect_literal())?),
            "#" => Value::Ver(Version::from_text(self.collect_literal())?), // FIX change literal to "v{VERSION}"? keep "#" for smth else
            _ => return Ok(None),
        };
        Ok(Some(V(value)))
    }

    fn match_reference(&mut self, piece: &str) -> Option<Token> {
        if REFERENCE.is_match(piece) {
            self.next();
            Some(V(Value::I(Id::new_reference(piece))))
        } else {
            None
        }
    }
}

lazy_static! {
    static ref SKIP: R = R::new(r"^(?i)(a|the|let|,|\t| |\?)+$").unwrap();
    static ref LITERAL_END: R = R::new(r"^(?i)(\.|\n|\t| |\?)+$").unwrap();
    static ref REFERENCE: R = R::new(r"^\p{Letter}+").unwrap();
    static ref NUMBER: R = R::new(r"^(\d+([\.,]\d+)?)$").unwrap();
    static ref TEXT: R = R::new(r#"^("|')$"#).unwrap();
}
