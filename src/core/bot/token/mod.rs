mod display;
mod tokens;
pub use tokens::*;
use crate::core::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    V(Value),
    T(Text),
    O(Op),
    C(Case),
    F(Flow),
    M(Modifier),
    S(Set),
    Being,
    This,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,       // X T
    Divide,    // X B
    Multiply,  // X B
    Substract, // X S
    Send,      // X T
    Filter,    // X B
    Collect,   // X B
    Read,      // X S
    Write,     // X T
    Sum,       // X ?B??
    Request,   // X ?S/?T => Trash?
    Sort,      // X ?B
    Expect,    // X ?S
    Mean,      // S
    Deviation, // S
    Sync,      // X B
    Show,      // X ?T
    Plot,      // X ?
    Sign,      // X ?B
    Verify,    // X ?B/S
    Predict,   // X S
}

#[derive(Debug, PartialEq)]
pub enum Case {
    And,
    More,
    Less,
    Not,
    Or,
    Any,
    Each,
}

#[derive(Debug, PartialEq)]
pub enum Flow {
    Break,
    Do,
    Else,
    End,
    For,
    If,
    Then,
    While,
    Return,
    ExpressionStart,
    ExpressionEnd,
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Binding,
    Selection,
    Targeting,
}

#[derive(Debug, PartialEq)]
pub enum Set {
    StructStart,
    StructEnd,
    ListStart,
    ListEnd,
}