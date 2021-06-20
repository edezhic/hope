use crate::core::*;
use std::iter::Peekable;
use Token::*;

pub struct Tokens {
    tokens: Vec<Token>,
    idx: usize,
}
impl Tokens {
    pub fn add(&mut self, token: Token) {
        if token != F(Flow::Break) || self.tokens.last().unwrap() != &F(Flow::Break) {
            self.tokens.push(token);
        }
    }
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        print!("{} ", token);
    }
}
