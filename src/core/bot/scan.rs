use super::vocabulary::*;
use crate::core::*;

impl Bot {
    pub fn scan(&mut self, text: Text) -> Result<Vec<Token>> {
        let mut pieces = text.split_by_word_bounds();
        let mut piece;
        let mut tokens = Vec::<Token>::new();
        let mut pattern = Pattern::None;

        loop {
            match pattern {
                Pattern::None => {
                    if let Some(s) = pieces.next() {
                        piece = s;
                    } else {
                        return Ok(tokens);
                    }
                    if self.vocab.skip(piece) {
                        continue;
                    } else if let Some(p) = self.vocab.check_pattern(piece) {
                        pattern = p;
                    } else if let Some(num) = self.vocab.number(piece) {
                        tokens.push(Token::Val(Value::Number(num)));
                    } else if let Some(t) = self.vocab.reserved(piece) {
                        tokens.push(t);
                    } else if let Some(t) = self.vocab.term(piece) {
                        tokens.push(t);
                    } else {
                        return Err(Error::ParsingError(format!(
                            r#"Unrecognized piece: '{}'"#,
                            piece
                        )));
                    }
                }
                Pattern::Comment => {
                    let mut comment = Text::empty();
                    while let Some(piece) = pieces.next() {
                        if self.vocab.comment_end(piece) {
                            tokens.push(Token::Comment(comment));
                            pattern = Pattern::None;
                            break;
                        } else {
                            comment.add(piece);
                        }
                    }
                }
                Pattern::Id => {
                    let mut id = Text::empty();
                    while let Some(piece) = pieces.next() {
                        if let Some(end) = self.vocab.literal_end(piece) {
                            tokens.push(Token::Val(Value::Id(Id::from_text(id)?)));
                            tokens.push(end);
                            pattern = Pattern::None;
                            break;
                        } else if self.vocab.whitespace(piece) {
                            tokens.push(Token::Val(Value::Id(Id::from_text(id)?)));
                            pattern = Pattern::None;
                            break;
                        } else {
                            id.add(piece);
                        }
                    }
                }
                Pattern::Seal => {
                    let mut seal = Text::empty();
                    while let Some(piece) = pieces.next() {
                        if let Some(end) = self.vocab.literal_end(piece) {
                            tokens.push(Token::Val(Value::Seal(Seal::from_text(seal)?)));
                            tokens.push(end);
                            pattern = Pattern::None;
                            break;
                        } else if self.vocab.whitespace(piece) {
                            tokens.push(Token::Val(Value::Seal(Seal::from_text(seal)?)));
                            pattern = Pattern::None;
                            break;
                        } else {
                            seal.add(piece);
                        }
                    }
                }
                Pattern::Text => {
                    // collect text until TEXT_END, add token and switch back to None
                    let mut text = Text::empty();
                    while let Some(piece) = pieces.next() {
                        if self.vocab.text_end(piece) {
                            tokens.push(Token::Val(Value::Text(text)));
                            pattern = Pattern::None;
                            break;
                        } else {
                            text.add(piece);
                        }
                    }
                }
                Pattern::Time => {
                    let mut time = Text::empty();
                    while let Some(piece) = pieces.next() {
                        if let Some(end) = self.vocab.literal_end(piece) {
                            tokens.push(Token::Val(Value::Time(Time::from_text(time)?)));
                            tokens.push(end);
                            pattern = Pattern::None;
                            break;
                        } else if self.vocab.whitespace(piece) {
                            tokens.push(Token::Val(Value::Time(Time::from_text(time)?)));
                            pattern = Pattern::None;
                            break;
                        } else {
                            time.add(piece);
                        }
                    }
                }
                Pattern::Version => {
                    // collect text until SPACE, add token Version.from_text, switch back to None
                    let mut version = Text::empty();
                    while let Some(piece) = pieces.next() {
                        if let Some(end) = self.vocab.literal_end(piece) {
                            tokens.push(Token::Val(Value::Version(Version::from_text(version)?)));
                            tokens.push(end);
                            pattern = Pattern::None;
                            break;
                        } else if self.vocab.whitespace(piece) {
                            tokens.push(Token::Val(Value::Version(Version::from_text(version)?)));
                            pattern = Pattern::None;
                            break;
                        } else {
                            version.add(piece);
                        }
                    }
                }
            }
        }
    }
}
