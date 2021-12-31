use crate::{Value, Text};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Token {
    V(Value),
    N(Text),
    O(Op),
    C(Command),
    M(Modifier),
    
    Being,
    This,
    And,
    Or,
    Any,
    Each,
    Less,
    More,
    Than,

    Break,
    Do,
    Else,
    End,
    For,
    If,
    Then,
    While,
    Return,

    ListStart,
    ListEnd,
    StructStart,
    StructEnd,
    FormulaStart,
    FormulaEnd,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Command {
    Add,       // X To
    Substract, // X From -> Remove/Delete?
    Send,      // X To
    Filter,    // X ?
    Sum,       // X ?
    Save,
    Request,   // X From
    Sort,      // X By
    Show,      // X ?
    Sign,      // X With(As?)
    Split,     // X By
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Op {
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
}
