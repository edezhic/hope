mod vocabulary;
mod pieces;

pub use vocabulary::Vocabulary;
pub use pieces::Pieces;

use crate::core::*;
use Token::{V, T};

impl Bot {
    pub fn translate(&self, s: &str) -> Result<Vec<Token>> {
        let text = Text::from_str(s);
        let mut pieces = Pieces::split(&text, &self.vocab);
        let mut tokens = vec![];
        while let Some(piece) = pieces.peek {
            if let Some(value) = self.vocab.match_value(piece, &mut pieces)? {
                tokens.push(V(value));
            } else if let Some(token) = self.vocab.match_token(piece) {
                pieces.next();
                tokens.push(token)
            } else if self.vocab.valid_term(piece) {
                pieces.next();
                tokens.push(T(Text::lowercase(piece)))
            } else {
                return Err(Error::ParsingError(format!(
                    r#"I don't know how to translate '{}'"#,
                    piece
                )));
            }
        }
        Ok(tokens)
    }
}


