use crate::core::*;
use core::fmt;

// Spec :: Section :: Tokens
// Spec X, Include X
// Section Y with { arg1: value1, arg2: value2, ... } `default values for args`
// Invoke Y (with { ... } `optional args override`)

#[derive(Debug)]
pub enum Token {
    Mod(Modifier),
    Op(Op),
    Ref(Value),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Assign,
    Await,
    Divide,
    Multiply,
    Send,
    Sign,
    Substract,
    Sum,
    Verify,
}

#[derive(Debug)]
pub enum Modifier {
    Binding,
    Break,
    Case(Case),
    Flow(Flow),
    Gap,
    Selector(Selector),
    Targeting,
}

#[derive(Debug)]
pub enum Case {
    And,
    Identical,
    Not,
    Or,
}

#[derive(Debug)]
pub enum Flow {
    Do,
    Else,
    End,
    For,
    If,
    Invoke,
    Then,
    When,
    While,
}

#[derive(Debug)]
pub enum Selector {
    Any,
    Each,
    Of,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Mod(modifier) => write!(f, "Mod:{:?}", modifier),
            Token::Op(op) => write!(f, "Op:{:?}", op),
            Token::Ref(value) => write!(f, "{}", value),
        }
    }
}
