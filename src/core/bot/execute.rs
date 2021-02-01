use std::{iter::Peekable, vec::IntoIter};

use crate::core::*;

impl Bot {
    pub fn execute(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut tokens = tokens.into_iter().peekable();
        let mut current_term: Option<Text> = None;

        while let Some(token) = tokens.next() {
            match token {
                Token::Term(term) => {
                    if !self.terms.contains(&term) {
                        self.terms.add(term.clone(), Value::default())
                    }
                    current_term = Some(term);
                }
                Token::Assign => {
                    self.result = Some(self.resolve(&mut tokens)?);
                    self.terms
                        .set(current_term.take().unwrap(), self.result.take().unwrap());
                }
                Token::Cmd(command) => {
                    self.result = command.execute(self.resolve(&mut tokens)?)?;
                }

                Token::Exp(Expression::Start) => {
                    // self.result = self.evaluate until exp::end
                }

                Token::Case(_) => {
                    // many different things
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn resolve(&mut self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        if let Some(token) = tokens.next() {
            if let Token::Val(value) = token {
                Ok(value)
            } else if let Token::This = token {
                if let Some(_) = self.result {
                    Ok(self.result.take().unwrap())
                } else {
                    Err(Error::Error("Result is undefined here"))
                }
            } else if let Token::Term(term) = token {
                self.select(term, tokens)
            } else if let Token::Col(Collection::ListStart) = token {
                self.collect_list(tokens)
            } else if let Token::Col(Collection::StructStart) = token {
                self.collect_struct(tokens)
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

    fn select(&self, term: Text, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
            let mut selectors: Vec<Text> = vec![term];
            let mut value: &Value;
            while let Some(Token::Mod(Modifier::Selection)) = tokens.peek() {
                tokens.next();
                if let Some(Token::Term(selector)) = tokens.next() {
                    selectors.push(selector);
                } else {
                    return Err(Error::Error("I can select only from terms"));
                }
            }
            let term = &selectors.pop().unwrap();
            if self.terms.contains(term) {
                value = self.terms.get(term).unwrap();
            } else {
                return Err(Error::ExecutionError(format!(
                    r#"Term: '{:?}' not found"#,
                    term
                )))
            }

            for selector in selectors.into_iter().rev() {
                if let Value::Structure(structure) = value {
                    if structure.contains(&selector) {
                        value = structure.get(&selector).unwrap()
                    } else {
                        return Err(Error::ExecutionError(format!(
                            r#"Term: '{:?}' not found"#,
                            term
                        )))
                    }
                } else {
                    return Err(Error::Error("I can select only from structures"))
                }
            }

        //Ok(self.terms.get(&term).unwrap().clone())
        Ok(value.clone())     
    }

    fn collect_struct(&mut self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        let mut structure = Structure::new();
        while let Some(token) = tokens.peek() {
            if let Token::Col(Collection::StructEnd) = token {
                break;
            } else if let Token::Next = token {
                tokens.next();
            } else if let Some(Token::Term(term)) = tokens.next() {
                if let Some(Token::Assign) = tokens.peek() {
                    tokens.next();
                    structure.add(term, self.resolve(tokens)?);
                } else {
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

    fn collect_list(&mut self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        let mut list = List::new();
        while let Some(token) = tokens.peek() {
            if let Token::Col(Collection::ListEnd) = token {
                break;
            } else if let Token::Next = token {
                tokens.next();
            } else {
                list.append(self.resolve(tokens)?);
            }
        }
        Ok(Value::List(list))
    }
}
