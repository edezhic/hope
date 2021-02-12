use crate::core::*;

#[derive(Debug)]
pub enum Token {
    Case(Case),
    Mod(Modifier),
    Op(Op),
    Ref(Value),
}

#[derive(Debug)]
pub enum Case {
    And,
    Any,
    Do,
    Each,
    Else,
    Identical,
    If,
    Not,
    Or,
    Stop,
    Then,
    When,
    While,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Await,
    Divide, 
    Multiply,
    Resolve,
    Sign,
    Send,
    Define,
    Substract,
    Sum,
    Verify,
}

#[derive(Debug)]
pub enum Modifier {
    Binding,
    Selection,
    Targeting,
}