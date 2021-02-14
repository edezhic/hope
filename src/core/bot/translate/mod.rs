mod collect;
mod lexeme;
mod read;
mod vocabulary;
use crate::core::*;
pub use lexeme::Lexeme;
pub use vocabulary::*;

impl Bot {
    pub fn translate(&self, text: Text) -> Result<Vec<Token>> {
        let mut pieces = &mut text.split_by_word_bounds().peekable();
        let mut tokens = Vec::<Token>::new();
        while let Some(_) = pieces.peek() {
            match self.read(pieces)? {
                Lexeme::None => {
                    pieces.next();
                }
                Lexeme::Comment(_) => (),
                Lexeme::Item(item) => tokens.push(Token::Ref(item)),
                Lexeme::List(list) => tokens.push(Token::Ref(Value::List(list))),
                Lexeme::Struct(structure) => tokens.push(Token::Ref(Value::Structure(structure))),
                Lexeme::Keyword(keyword) => tokens.push(keyword),
                Lexeme::Reference(reference) => tokens.push(Token::Ref(reference)),
                Lexeme::Command(command) => tokens.extend(command),
            }
        }
        Ok(tokens)
    }
}
