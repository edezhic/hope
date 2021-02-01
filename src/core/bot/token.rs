use crate::core::*;

#[derive(Debug)]
pub enum Token {
    Assign,
    Case(Case),
    Cmd(Command),
    Col(Collection),
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
pub enum Collection { 
    End,
    Start,
}

#[derive(Debug)]
pub enum Expression { 
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