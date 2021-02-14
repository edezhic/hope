use crate::core::*;

#[derive(Debug)]
pub enum Lexeme {
    None,
    Command(Vec<Token>),
    Comment(Text),
    Item(Value),
    Keyword(Token),
    List(List),
    Reference(Value),
    Struct(Structure),
}