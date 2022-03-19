use crate::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Token {
    Value(Value),
    Term(Text),
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
pub enum Command { // FIX
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
}
