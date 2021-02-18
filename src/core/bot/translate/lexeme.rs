use crate::core::*;

#[derive(Debug)]
pub enum Lexeme {
    Command(Vec<Token>),
    Comment(Text),
    Value(Value),
    Keyword(Token),
    Reference(Value),
}