use std::{iter::Peekable, vec::IntoIter};
use crate::core::*;

impl Bot {
    pub fn collect_struct(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        let mut structure = Structure::new();
        while let Some(token) = tokens.peek() {
            if let Token::Mod(Modifier::StructEnd) = token {
                break;
            } else if let Some(Token::Term(term)) = tokens.next() {
                if let Some(Token::Mod(Modifier::Assign)) = tokens.peek() {
                    tokens.next();
                    structure.set(term, self.reference(tokens)?.clone());
                } else {
                    let value;
                    if self.terms.contains(&term) {
                        value = self.terms.get(&term).unwrap().clone();
                    } else {
                        value = Value::flag();
                    }
                    structure.set(term, value);
                }
            }
        }
        Ok(Value::Structure(structure))
    }

    pub fn collect_list(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        let mut list = List::new();
        while let Some(token) = tokens.peek() {
            if let Token::Mod(Modifier::ListEnd) = token {
                break;
            } else {
                list.append(self.reference(tokens)?.clone());
            }
        }
        Ok(Value::List(list))
    }

    pub fn expect(&self, tokens: &mut Peekable<IntoIter<Token>>, token: Token) -> Result<()> {
        if let Some(token) = tokens.next() {
            Ok(())
        } else {
            Err(Error::Error("Expected target value"))
        }
    }
}