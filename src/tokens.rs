use crate::{Token::*, *};
use core::slice::Iter;
use std::{iter::Peekable, vec::IntoIter};

pub struct Tokens<'a> {
    iter: Peekable<Iter<'a, Token>>,
    pub peek: Option<&'a Token>,
}
impl<'a> Tokens<'a> {
    pub fn init(vec: &'a Vec<Token>) -> Result<Tokens> {
        let mut iter = vec.iter().peekable();
        let mut tokens = Tokens { iter, peek: None };
        tokens.update_peek();
        Ok(tokens)
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.iter.next();
        self.update_peek();
        self.peek
    }

    fn update_peek(&mut self) {
        if let Some(token) = self.iter.peek() {
            self.peek = Some(*token)
        } else {
            self.peek = None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Being,
    This,
    And,
    Or,

    V(Value),
    N(Text),
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
    With,
    By,
    Of,
    From,
    To,
    In,
    At,
}

use core::fmt;
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Being => write!(f, "="),
            Token::O(op) => write!(f, "{:#?}", op),
            Token::V(v) => write!(f, "{}", v),
            Token::F(flow) => match flow {
                Flow::Break => write!(f, "."),
                _ => write!(f, "{:#?}", flow),
            },
            Token::N(n) => write!(f, "{}", n),
            Token::This => write!(f, "this"),
            Token::C(c) => write!(f, "{:#?}", c),
            Token::S(s) => write!(f, "{:#?}", s),
            Token::FormulaStart => write!(f, "("),
            Token::FormulaEnd => write!(f, ")"),
            Token::StructStart => write!(f, "{{"),
            Token::StructEnd => write!(f, "}}"),
            Token::ListStart => write!(f, "["),
            Token::ListEnd => write!(f, "]"),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            //_ => write!(f, "???"),
        }
    }
}

pub fn print_tokens(tokens: &Vec<Token>) {
    print!("-----: ");
    for token in tokens {
        print!("{} ", token);
    }
    println!("");
}
