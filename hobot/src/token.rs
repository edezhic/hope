use crate::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Token {
    Value(Value),
    Op(Operation),
    Cmd(Command),
    Mod(Modifier),
    
    Being,
    This,

    And,
    Or,
    
    Any,
    Each,
    Less,
    More,
    Than,
    Contains,

    Break,
    Do,
    Else,
    End,
    For,
    If,
    Then,
    While,
    Where,
    Return,
    Match,
    Try,
    Panic,

    ListStart,
    ListEnd,
    StructStart,
    StructEnd,
    FormulaStart,
    FormulaEnd,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Command {
    Add,       // To
    Substract, // From -> Remove/Delete?
    Send,      // To
    Filter,    // ?
    Sum,       // ?
    Store,     // At
    Request,   // From
    Sort,      // By
    Show,      // ?
    Sign,      // With(As?)
    Group,     // By -> Group by?
    Select,    // From
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Multiplication,
    Division,
    Mean,
    Deviation,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Modifier {
    With,
    By,
    Of,
    From,
    To,
    In,
    At,
    As,
}
