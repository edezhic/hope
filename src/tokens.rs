use crate::{*, Token::*};
use core::{slice::Iter, iter::Peekable};

pub struct Tokens<'a> {
    iter: Peekable<Iter<'a, Token>>,
    pub peek: Option<&'a Token>,
}
impl<'a> Tokens<'a> {
    pub fn init(vec: &'a Vec<Token>) -> Result<Tokens> {
        let mut iter = vec.into_iter().peekable();
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
    V(Value),
    N(Text),
    O(Op),
    C(Command),
    // T(Type) Number/Text/Id/...?
    // S(Script)/Ref?
    
    Being,
    This,
    And,
    Or,
    FormulaStart,
    FormulaEnd,
    StructStart,
    StructEnd,
    ListStart,
    ListEnd,

    Break,
    Do,
    Else,
    End,
    For,
    If,
    Then,
    While,
    Return,
    Any,
    Each,
    
    With, // => Back to Modifier?
    By,
    Of,
    From,
    To,
    In,
    At,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Add,       // X T
    Substract, // X S; -> Remove/Delete?
    Send,      // X T
    Filter,    // X B?
    Sum,       // X
    Request,   // X S
    Sort,      // X B
    Show,      // X T
    Sign,      // X B
    Split,     // X B?
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

pub fn print_tokens(tokens: &Vec<Token>) {
    print!("-----: ");
    for token in tokens {
        match token {
            O(op) => print!("{:#?}", op),
            V(v) => print!("{}", v),
            N(n) => print!("{}", n),
            C(c) => print!("{:#?}", c),
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
