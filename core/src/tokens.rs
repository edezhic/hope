use crate::{*, Token::*};
use core::{slice::Iter, iter::Peekable};

pub struct Tokens<'a> {
    iter: Peekable<Iter<'a, (usize, Token)>>,
    pub peek: Option<&'a (usize, Token)>,
}
impl<'a> Tokens<'a> {
    pub fn init(vec: &'a Vec<(usize, Token)>) -> Result<Tokens> {
        let mut iter = vec.into_iter().peekable();
        let mut tokens = Tokens { iter, peek: None };
        tokens.update_peek();
        Ok(tokens)
    }

    pub fn next(&mut self) -> Option<&(usize, Token)> {
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Token {
    V(Value),
    N(Text),
    O(Op),
    C(Command),
    M(Modifier),
    
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
pub enum Command {
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
pub enum Op {
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

pub fn print_tokens(tokens: &Vec<(usize, Token)>) {
    print!("|||||||: ");
    for (index, token) in tokens {
        match token {
            O(op) => print!("{:#?}", op),
            V(v) => print!("{}", v),
            N(n) => print!("{}", n),
            C(c) => print!("{:#?}", c),
            M(m) => print!("{:#?}", m),
            Break => print!("."),
            FormulaStart => print!("("),
            FormulaEnd => print!(")"),
            StructStart => print!("{{"),
            StructEnd => print!("}}"),
            ListStart => print!("["),
            ListEnd => print!("]"),
            _ => print!("{:#?}", token),
        }
        print!(" ");
    }
    println!("");
}
