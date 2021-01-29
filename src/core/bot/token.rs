use crate::core::*;

#[derive(Debug)]
pub enum Token {
    Case(Case),
    Cmd(Command),
    Comment(Text),
    Exp(Expression),
    Mod(Modifier),
    Term(Text),
    This,
    Val(Value),
}

#[derive(Debug)]
pub enum Case {
    And,
    Any,
    Each,
    Else,
    Equal,
    If,
    Not,
    Or,
    Repeat,
    Stop,
    Then,
    When,
    While,
}

#[derive(Debug)]
pub enum Expression { 
    Assign,
    Divide,
    End,
    Minus,
    Multiply,
    Plus,
    Start,
}

#[derive(Debug)]
pub enum Modifier {
    Binding,
    New,
    Next,
    Selection,
    Targeting,
}