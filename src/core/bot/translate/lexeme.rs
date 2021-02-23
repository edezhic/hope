use crate::core::*;

#[derive(Debug)]
pub enum Lexeme {
    Command(Vec<Token>),
    Comment(Text),
    Value(Value),
    Token(Token),
    Reference(Value),
    ???
    If,
    Then,
    Else,
    While,
    Do,
    When,
    End,
    For,
}