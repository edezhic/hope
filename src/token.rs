use crate::*;
use std::{iter::Peekable, vec::IntoIter};

pub struct Tokens<'a> {
    iter: Peekable<IntoIter<Token>>,
    pub peek: Option<&'a Token> 
}
impl<'a> Tokens<'a> {
    pub fn init(vec: Vec<Token>) -> Tokens<'a> {
        let mut iter = vec.into_iter().peekable();
        let mut tokens = Tokens {
            iter,
            peek: None
        };
        tokens.peek = tokens.iter.peek();
        //tokens.update_peek();
        tokens
    }

    pub fn next(&'a mut self) -> Option<&'a Token> {
        self.iter.next();
        self.peek
    }

}

#[derive(Debug, PartialEq)]
pub enum Token {
    Val(Value),
    Term(Text),
    O(Op),
    Cmd(Command),
    C(Case),
    F(Flow),
    Mod(Modifier),
    And,
    Or,
    Being,
    This,
    FormulaStart,
    FormulaEnd,
    StructStart,
    StructEnd,
    ListStart,
    ListEnd,
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
    Show,      // X T
    Plot,      // X
    Sign,      // X B
    Check,     // X ?B/S
    Predict,   // X S
    Split,     // X B

    Custom {
        name: Text, 
        arg: Option<Modifier>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Multiplication,
    Division,
    Mean,
    Deviation,
}

#[derive(Debug, PartialEq)]
pub enum Case {
    More,
    Less,
    Not,
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
    
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Binding,
    Selection,
    Targeting,
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
                _ => write!(f, "F"),
            },
            Token::Mod(modifier) => match modifier {
                Modifier::Binding => write!(f, "b"),
                Modifier::Selection => write!(f, "s"),
                Modifier::Targeting => write!(f, "t"),
            },
            Token::Term(_) => write!(f, "T"),
            Token::This => write!(f, "_"),
            Token::Cmd(_) => write!(f, "Cmd"),
            Token::FormulaStart => write!(f, "("),
            Token::FormulaEnd => write!(f, ")"),
            Token::StructStart => write!(f, "{{"),
            Token::StructEnd => write!(f, "}}"),
            Token::ListStart => write!(f, "["),
            Token::ListEnd => write!(f, "]"),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
        }
    }
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{} ", token);
    }
}