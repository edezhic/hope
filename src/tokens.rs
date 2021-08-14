use crate::{Token::*, *};
use core::slice::Iter;
use std::{iter::Peekable, vec::IntoIter};

pub struct Tokens<'a> {
    iter: Peekable<IntoIter<Token>>,
    pub peek: Option<&'a Token>,
}
impl<'a> Tokens<'a> {
    pub fn init(s: &'a str) -> Result<Tokens<'a>> {
        let text = Text::from_str(s);
        let mut pieces = Pieces::split(&text);
        let mut vec = vec![];
        while let Some(piece) = pieces.peek {
            vec.push(match_token(&mut pieces)?);
        }

        print!("-----: ");
        print_tokens(&vec);
        println!("");

        let mut iter = vec.into_iter().peekable();
        let mut tokens = Tokens { iter, peek: None };
        //tokens.update_peek();
        Ok(tokens)
    }

    pub fn next(&'a mut self) -> Option<&'a Token> {
        self.iter.next();
        self.peek
    }
    fn update_peek(&'a mut self) {
        self.peek = self.iter.peek();
        
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Being,
    This,
    And,
    Or,

    V(Value),
    T(Text),
    O(Op),
    C(Command),
    F(Flow),
    S(Specifier),
    // T(Type) Number/Text/Id/...?
    FormulaStart,
    FormulaEnd,
    StructStart,
    StructEnd,
    ListStart,
    ListEnd,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Init,      //
    Add,       // X T
    Substract, // X S
    Send,      // X T
    Filter,    // X B?
    Sum,       // X
    Request,   // X S
    Sort,      // X B
    Show,      // X T
    Sign,      // X B
    Split,     // X B?

    Custom { id: Id, arg: Option<Specifier> },
}

#[derive(Debug, PartialEq)]
pub enum Op {
    More,
    Less,
    Not,
    Empty,

    Plus,
    Minus,
    Multiplication,
    Division,
    Mean,
    Deviation,
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
pub enum Specifier {
    Any,
    Each,
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
            Token::V(_) => write!(f, "V"),
            Token::F(flow) => match flow {
                Flow::Break => write!(f, "."),
                _ => write!(f, "F"),
            },
            Token::T(_) => write!(f, "T"),
            Token::This => write!(f, "_"),
            Token::C(_) => write!(f, "C"),
            Token::FormulaStart => write!(f, "("),
            Token::FormulaEnd => write!(f, ")"),
            Token::StructStart => write!(f, "{{"),
            Token::StructEnd => write!(f, "}}"),
            Token::ListStart => write!(f, "["),
            Token::ListEnd => write!(f, "]"),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            _ => write!(f, "???"),
        }
    }
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{} ", token);
    }
}
