use crate::core::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Val(Value),
    Term(Text),
    O(Op),
    Cmd(Command),
    C(Case),
    F(Flow),
    Mod(Modifier),
    S(Set),
    Being,
    This,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Add,       // X T
    Divide,    // X B
    Multiply,  // X B
    Substract, // X S
    Send,      // X T
    Filter,    // X B
    Collect,   // X B
    Read,      // X S
    Write,     // X T
    Sum,       // X 
    Request,   // X S
    Sort,      // X B
    Expect,    // X S => trash?
    Mean,      // => Op? S 
    Deviation, // => Op? S
    Sync,      // X B
    Show,      // X ?T
    Plot,      // X ?
    Sign,      // X ?B
    Check,     // X ?B/S
    Predict,   // X S
    Split,     // X B

    Custom {
        term: Text, 
        arg: Option<Modifier>, // ???
        // tokens/algorithm?
    },
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Multiplication,
    Division,
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
    Has,
    Empty,
    Where,
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
    None,
}

#[derive(Debug, PartialEq)]
pub enum Set {
    StructStart,
    StructEnd,
    ListStart,
    ListEnd,
}


use core::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Being => write!(f, "="),
            Token::O(op) => write!(f, "O"),
            Token::Val(value) => write!(f, "V"),
            Token::C(case) => write!(f, "C"),
            Token::F(flow) => match flow {
                Flow::Break => write!(f, "."),
                Flow::ExpressionStart => write!(f, "("),
                Flow::ExpressionEnd => write!(f, ")"),
                _ => write!(f, "F"),
            },
            Token::Mod(modifier) => match modifier {
                Modifier::Binding => write!(f, "b"),
                Modifier::Selection => write!(f, "s"),
                Modifier::Targeting => write!(f, "t"),
                Modifier::None => write!(f, ""),
            },
            Token::S(set) => match set {
                Set::StructStart => write!(f, "{{"),
                Set::StructEnd => write!(f, "}}"),
                Set::ListStart => write!(f, "["),
                Set::ListEnd => write!(f, "]"),
            },
            Token::Term(_) => write!(f, "T"),
            Token::This => write!(f, "_"),
            Token::Cmd(_) => write!(f, "Cmd"),
        }
    }
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{} ", token);
    }
}