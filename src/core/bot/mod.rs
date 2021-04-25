mod token;
mod vocabulary;

use crate::core::*;
pub use token::*;
pub use vocabulary::*;

use Token::{T, V};

pub struct Bot {
    vocab: Vocabulary,
}

impl Bot {
    pub fn init() -> Result<Bot> {
        Ok(Bot {
            vocab: Vocabulary::english()?,
        })
    }
    pub fn debug(&self, s: &str) -> Result<()> {
        println!("{} ", s);
        let tokens = self.translate(s)?;
        print!("-----: ");
        tokens.print();
        println!("");
        //let graph = self.link(tokens);
        Ok(())
    }

    pub fn translate(&self, s: &str) -> Result<Tokens> {
        let text = Text::from_str(s);
        let mut pieces = Pieces::split(&text, &self.vocab);
        let mut tokens = Tokens::new();
        while let Some(piece) = pieces.peek {
            if let Some(value) = self.vocab.match_value(piece, &mut pieces)? {
                tokens.add(V(value));
            } else if let Some(token) = self.vocab.match_token(piece) {
                tokens.add(token);
                pieces.next();
            } else if self.vocab.valid_term(piece) {
                tokens.add(T(Text::lowercase(piece)));
                pieces.next();
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
