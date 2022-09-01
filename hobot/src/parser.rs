use crate::*;

pub fn parse(s: &str) -> Result<Vec<IndexedToken>> {
    let text = Text::from_str(s);
    let pieces = text.split_by_word_bounds().peekable();
    let mut parser = Parser { pieces, peek: None };
    parser.update_peek();
    let mut indexed_tokens = vec![];
    while let Some((index, piece)) = parser.peek {
        if let Some(value) = parser.match_value(piece)? {
            indexed_tokens.push(IndexedToken {
                index,
                token: value,
            });
        } else if let Some(token) = Token::matches(piece) {
            indexed_tokens.push(IndexedToken { index, token });
            parser.next();
        } else if let Some(term) = parser.validate_term(piece) {
            indexed_tokens.push(IndexedToken { index, token: term });
        } else {
            return Err(Parsing(format!(
                r#"I don't know how to parse '{:?}'"#,
                piece
            )));
        }
    }
    Ok(indexed_tokens)
}

pub struct Parser<'a> {
    pieces: Peekable<UWordBoundIndices<'a>>,
    pub peek: Option<(usize, &'a str)>,
}
impl<'a> Parser<'a> {
    fn update_peek(&mut self) {
        self.skip_ignored();
        if let Some(piece) = self.pieces.peek() {
            self.peek = Some(*piece)
        } else {
            self.peek = None
        }
    }

    fn next(&mut self) -> Option<(usize, &'a str)> {
        self.pieces.next();
        self.update_peek();
        self.peek
    }

    fn collect_text(&mut self) -> Text {
        self.pieces.next();
        let mut text = Text::empty();
        while let Some((_, piece)) = self.pieces.peek() {
            if TEXT.is_match(piece) {
                self.pieces.next();
                break;
            } else {
                text.add(piece);
                self.pieces.next();
            }
        }
        self.update_peek();
        text
    }

    fn collect_literal(&mut self) -> Text {
        self.pieces.next();
        let mut text = Text::empty();
        while let Some((_, piece)) = self.pieces.peek() {
            if LITERAL_END.is_match(piece) {
                break;
            } else {
                text.add(piece);
                self.pieces.next();
            }
        }
        self.update_peek();
        text
    }

    fn skip_ignored(&mut self) {
        while let Some((_, piece)) = self.pieces.peek() {
            if IGNORED.is_match(piece) {
                self.pieces.next();
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

    fn validate_term(&mut self, piece: &str) -> Option<Token> {
        if TERM.is_match(piece) {
            self.next();
            Some(Term(Text::from_str(piece)))
        } else {
            None
        }
    }
}

lazy_static! {
    static ref IGNORED: Regex = Regex::new(r"^(?i)(a|the|let|,|\t| |\?)+$").unwrap();
    static ref LITERAL_END: Regex = Regex::new(r"^(?i)(\.|\n|\t| |\?)+$").unwrap();
    static ref TERM: Regex = Regex::new(r"^\p{Letter}+").unwrap();
    static ref NUMBER: Regex = Regex::new(r"^(\d+([\.,]\d+)?)$").unwrap();
    static ref TEXT: Regex = Regex::new(r#"^("|')$"#).unwrap();
}
