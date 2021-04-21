use crate::core::*;
use core::fmt;

#[derive(Debug)]
pub enum Token {
    V(Value),
    T(Text),
    O(Op),
    C(Case),
    F(Flow),
    M(Modifier),
    S(Set),
    Being,
}

#[derive(Debug)]
pub enum Op {
    Add,          // X T
    Divide,       // X B
    Multiply,     // X B
    Substract,    // X S
    Send,         // X T
    Filter,       // X B
    Collect,      // X B
    Exponentiate, // X T
    Read,         // X S
    Write,        // X T
    Sum,          // X ?B??
    Request,      // X ?S/?T
    Sort,         // X ?B
    Expect,       // X ?S
    Mean,         // S
    Deviation,    // S

    Sync, // X B

    Show, // X ?T
    Plot, // X ?

    Sign,   // X ?B
    Verify, // X ?B/S

    Predict, // X S
    Model,   // X
}

#[derive(Debug)]
pub enum Case {
    And,
    More,
    Less,
    Not,
    Or,
    Any,
    Each,
}

#[derive(Debug)]
pub enum Flow {
    Break,
    Do,
    Else,
    End,
    Expect,
    For,
    If,
    Invoke,
    Then,
    While,
    ExpressionStart,
    ExpressionEnd,
}

#[derive(Debug)]
pub enum Modifier {
    Binding,
    Selection,
    Targeting,
}

#[derive(Debug)]
pub enum Set {
    StructStart,
    StructEnd,
    ListStart,
    ListEnd,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Being => write!(f, "="),
            Token::O(op) => write!(f, "O"),
            Token::V(value) => write!(f, "V"),
            Token::C(case) => write!(f, "C"),
            Token::F(flow) => match flow {
                Flow::Break => write!(f, "."),
                _ => write!(f, "F"),
            },
            Token::M(modifier) => match modifier {
                Modifier::Binding => write!(f, "_"),
                Modifier::Selection => write!(f, "_"),
                Modifier::Targeting => write!(f, "_"),
            },
            Token::S(set) => match set {
                Set::StructStart => write!(f, "{{"),
                Set::StructEnd => write!(f, "}}"),
                Set::ListStart => write!(f, "["),
                Set::ListEnd => write!(f, "]"),
            },
            Token::T(_) => write!(f, "T"),
        }
    }
}
