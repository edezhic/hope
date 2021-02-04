use crate::core::*;

#[derive(Debug)]
pub enum Token {
    Case(Case),
    Cmd(Command),
    Mod(Modifier),
    Op(Op),
    Result,
    Term(Text),
    Val(Value),
}

#[derive(Debug)]
pub enum Case {
    And,
    Any,
    Do,
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
pub enum Command {
    Await,
    Seal,
    Send,
    Set,
    Show,
    Sum,
    Verify,
}

#[derive(Debug)]
pub enum Op { 
    Divide,
    End,
    Minus,
    Multiply,
    Plus,
}

#[derive(Debug)]
pub enum Modifier {
    Assign,
    Binding,
    ExpEnd,
    ExpStart,
    ListEnd,
    ListStart,
    Selection,
    StructEnd,
    StructStart,
    Targeting,
}