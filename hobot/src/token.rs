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
    pub fn take_ref(&mut self) -> Result<Id> {
        if let V(I(id)) = self.take() {
            if id.scheme.is_ref() {
                Ok(id)
            } else {
                Err(Message("Expected ref scheme in id while taking"))
            }
        } else {
            Err(Message("Expected Id when taking reference"))
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
    pub fn next(&mut self) -> Option<(usize, Token)> {
        self.iter.next()
    }
    pub fn until(&mut self, token: Token) -> bool {
        if *self.peek() != token {
            true
        } else {
            self.next();
            false
        }
    }
    pub fn expect(&mut self, token: Token) -> Result<()> {
        if let Some((_, next_token)) = self.iter.next() {
            if token == next_token {
                return Ok(());
            }
        }
        Err(UnexpectedToken(token))
    }

}

impl Token {
    pub fn is_ref(&self) -> bool {
        if let V(I(id)) = self {
            return id.scheme.is_ref();
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Token {
    Then,
    And,
    #[dont_match]
    Input,
    Or,
    #[regex = r"^(?i)(result|this|it|that)$"]
    This,
    #[regex = r"^(?i)(:|=|is|are|equal)$"]
    Be,
    #[regex = r"^\.$"]
    Dot,
    #[regex = r"^(\n|\p{Zl})$"]
    Linebreak,
    #[regex = r"^(\]|\})$"]
    CollectionEnd,
    A(Algebra),
    C(Control),
    S(Selector),
    F(Function),
    V(Value),
    P(Preposition),
    R(Relation),
    #[dont_match]
    Script(Id),
    #[dont_match]
    Model(Id),
    // + static?
    // + T(Type)
    // + Script and Model here?
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Selector {
    Where,
    Any,
    Each,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Relation {
    Than,
    Less,
    More,
    Contains,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
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
