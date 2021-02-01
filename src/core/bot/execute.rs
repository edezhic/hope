use std::{iter::Peekable, vec::IntoIter};

use crate::core::*;

impl Bot {
    pub fn execute(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut iter = tokens.into_iter().peekable();
        let mut current_term: Option<Text> = None;

        while let Some(token) = iter.next() {
            match token {
                Token::Term(term) => {
                    if !self.terms.contains(&term) {
                        self.terms.add(term.clone(), Value::default())
                    }
                    current_term = Some(term);
                }
                Token::Assign => {
                    self.result = Some(self.resolve(&mut iter)?);
                    self.terms
                        .set(current_term.take().unwrap(), self.result.take().unwrap());
                }
                Token::Cmd(command) => {
                    self.result = command.execute(self.resolve(&mut iter)?)?;
                }

                Token::Exp(Expression::Start) => {
                    // value = evaluate until exp::end
                }

                Token::Case(_) => {
                    // many different things
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn resolve(&mut self, iter: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        if let Some(token) = iter.next() {
            if let Token::Val(value) = token {
                Ok(value)
            } else if let Token::This = token {
                if let Some(_) = self.result {
                    Ok(self.result.take().unwrap())
                } else {
                    Err(Error::Error("Result is undefined here"))
                }
                
            } else if let Token::Term(term) = token {
                if self.terms.contains(&term) {
                    Ok(self.terms.get(&term).unwrap().clone())
                } else {
                    Err(Error::ExecutionError(format!(
                        r#"Unknown term: '{:?}'"#,
                        term
                    )))
                }
            } else if let Token::Col(Collection::ListStart) = token {
                self.collect_list(iter)
            } else if let Token::Col(Collection::StructStart) = token {
                self.collect_struct(iter)
            } else {
                Err(Error::ExecutionError(format!(
                    r#"Cannot evaluate: '{:?}'"#,
                    token
                )))
            }
        } else {
            Err(Error::Error("Expected Value"))
        }
    }

    fn collect_struct(&mut self, iter: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        let mut structure = Structure::new();
        while let Some(token) = iter.peek() {
            if let Token::Col(Collection::StructEnd) = token {
                break;
            } else if let Token::Next = token {
                iter.next();
            } else if let Some(Token::Term(term)) = iter.next() {
                if let Some(Token::Assign) = iter.peek() {
                    iter.next();
                    structure.add(term, self.resolve(iter)?);
                } else {
                    // clone, add
                    let value;
                    if self.terms.contains(&term) {
                        value = self.terms.get(&term).unwrap().clone();
                    } else {
                        value = Value::default();
                    }
                    structure.add(term, value);
                }
                
            }
        }
        Ok(Value::Structure(structure))
    }

    fn collect_list(&mut self, iter: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        let mut list = List::new();
        while let Some(token) = iter.peek() {
            if let Token::Col(Collection::ListEnd) = token {
                break;
            } else if let Token::Next = token {
                iter.next();
            } else {
                list.append(self.resolve(iter)?);
            }
        }
        Ok(Value::List(list))
    }
}
