use crate::{Value::*, *};
use std::{iter::Peekable, vec::IntoIter};

pub struct TokensIterator {
    iter: Peekable<IntoIter<(usize, Token)>>,
}
impl TokensIterator {
    pub fn init(tokens: Vec<(usize, Token)>) -> Result<TokensIterator> {
        if tokens.len() > 0 {
            Ok(TokensIterator {
                iter: tokens.into_iter().peekable(),
            })
        } else {
            Err(Message("Script cannot be empty"))
        }
    }
    pub fn take(&mut self) -> Token {
        self.iter.next().unwrap().1
    }
    pub fn take_prep(&mut self) -> Result<Preposition> {
        if let P(preposition) = self.take() {
            Ok(preposition)
        } else {
            Err(Message("Expected modifier"))
        }
    }
    pub fn take_id(&mut self) -> Result<Id> {
        if let V(I(id)) = self.take() {
            // && id.is_ref()
            Ok(id)
        } else {
            Err(Message("Expected reference"))
        }
    }
    pub fn remain(&mut self) -> bool {
        if let Some(_) = self.iter.peek() {
            return true;
        }
        false
    }
    pub fn peek(&mut self) -> &Token {
        &self.iter.peek().unwrap().1
    }
    pub fn next(&mut self) {
        self.iter.next();
    }
}

impl Token {
    pub fn is_ref(&self) -> bool {
        if let Token::V(value) = self {
            if value.is_ref() {
                return true;
            }
        }
        false
    }
    pub fn is_function(&self) -> bool {
        if let Token::F(_) = self {
            return true;
        }
        false
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Token {
    And,
    Or,
    #[regex = r"^(?i)(result|this|it|that)$"]
    This,
    #[regex = r"^(?i)(:|=|is|are|equal)$"]
    Be,
    #[regex = r"^\.$"]
    Dot,
    #[regex = r"^(\n|\p{Zl})$"]
    Newline,
    #[regex = r"^(\]|\})$"]
    CollectionEnd,
    V(Value),
    A(Algebra),
    F(Function),
    P(Preposition),
    R(Relation),
    S(Selector),
    C(Control),
    // + static?
    // + T(Type)
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Control {
    Do,
    Else,
    If,
    Then,
    While,
    Return,
    Match,
    Try,
    Panic,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Selector {
    Where,
    Any,
    Each,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Relation {
    Than,
    Less,
    More,
    Contains,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Function {
    Add,       // To
    Substract, // From -> Remove/Delete?
    Filter,    // ?
    Sum,       // ?
    Send,      // To
    Get,       // ?
    Sort,      // By
    Show,      // ?
    Sign,      // With(As?)
    Group,     // By -> Group by?
    Select,    // From
    // Join?
    Script {}, // for user-defined functions
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Algebra {
    #[regex = r"^\($"]
    Start,
    #[regex = r"^\)$"]
    End,
    #[regex = r"^\+$"]
    Plus,
    #[regex = r"^\-$"]
    Minus,
    #[regex = r"^\*$"]
    Multiplication,
    #[regex = r"^/$"]
    Division,
    Mean,
    Deviation,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Preposition {
    For,
    With,
    By,
    Of,
    From,
    To,
    In,
    At,
    As,
}
