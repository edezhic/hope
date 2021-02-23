mod collect;
mod lexeme;
mod read;
mod vocabulary;

use crate::core::*;
pub use lexeme::Lexeme;
pub use vocabulary::Vocabulary;

impl Bot {
    pub fn translate(&self, text: Text) -> Result<Vec<Token>> {
        let mut pieces = &mut text.split_by_word_bounds().peekable();
        let mut tokens = Vec::<Token>::new();
        while let Some(_) = pieces.peek() {
            match self.read(pieces)? {
                Lexeme::Comment(_) => (),
                Lexeme::Command(command) => tokens.extend(command),
                Lexeme::Token(token) => tokens.push(token),
                Lexeme::Reference(reference) => tokens.push(Token::Ref(reference)),
                Lexeme::Value(item) => tokens.push(Token::Ref(item)),
            }
        }
        Ok(tokens)
    }
}
