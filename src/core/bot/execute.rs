use std::{iter::Peekable, vec::IntoIter};
use crate::core::*;

impl Bot {
    pub fn execute(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut tokens = &mut tokens.into_iter().peekable();
        let mut current_term: Option<Text> = None;

        while let Some(token) = tokens.next() {
            match token {
                Token::Term(term) => {
                    if !self.terms.contains(&term) {
                        self.terms.set(term.clone(), Value::default())
                    }
                    current_term = Some(term);
                }
                Token::Assign => {
                    let term = current_term.take().unwrap();
                    let value = self.collect_value(tokens)?;
                    self.terms.set(term, value);
                }
                Token::Cmd(command) => {
                    match command {
                        Command::Set => { 
                            let reference = self.collect_reference(tokens)?;
                            Bot::expect(tokens, Token::Mod(Modifier::Targeting))?;
                            let value = self.collect_value(tokens)?;
                            self.result = Command::set(reference, value)?;
                        }
                        Command::Show => {
                            let reference = self.collect_reference(tokens)?;
                            self.result = Command::show(reference)?;
                        }
                        Command::Sum => {
                            let reference = self.collect_reference(tokens)?;
                            self.result = Command::sum(reference)?;
                        }
                        _ => {
                            return Err(Error::ExecutionError(format!(
                                r#"I don't know how to '{:?}'"#,
                                command
                            )));
                        }
                    }
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

    fn collect_reference(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<&Value> {
        if let Some(token) = tokens.next() {
            if let Token::This = token {
                Ok(&self.result)
            } else if let Token::Term(term) = token {
                Ok(self.select(term, tokens)?)
            } else {
                Err(Error::ExecutionError(format!(
                    r#"Cannot reference: '{:?}'"#,
                    token
                )))
            }
        } else {
            Err(Error::Error("Expected reference"))
        }
    }

    fn collect_value(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        if let Some(token) = tokens.next() {
            if let Token::Val(value) = token {
                Ok(value)
            } else if let Token::This = token {
                Ok(self.result.clone())
            } else if let Token::Term(term) = token {
                Ok(self.select(term, tokens)?.clone())
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

    fn select(&self, term: Text, tokens: &mut Peekable<IntoIter<Token>>) -> Result<&Value> {
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
            )));
        }

        for selector in selectors.into_iter().rev() {
            if let Value::Structure(structure) = value {
                if structure.contains(&selector) {
                    value = structure.get(&selector).unwrap()
                } else {
                    return Err(Error::ExecutionError(format!(
                        r#"Term '{:?}' of '{:?}' not found"#,
                        term, selector
                    )));
                }
            } else {
                return Err(Error::Error("I can select only from structures"));
            }
        }
        Ok(value)
    }

    fn collect_struct(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        let mut structure = Structure::new();
        while let Some(token) = tokens.peek() {
            if let Token::Col(Collection::StructEnd) = token {
                break;
            } else if let Token::Next = token {
                tokens.next();
            } else if let Some(Token::Term(term)) = tokens.next() {
                if let Some(Token::Assign) = tokens.peek() {
                    tokens.next();
                    structure.set(term, self.collect_value(tokens)?.clone());
                } else {
                    let value;
                    if self.terms.contains(&term) {
                        value = self.terms.get(&term).unwrap().clone();
                    } else {
                        value = Value::default();
                    }
                    structure.set(term, value);
                }
            }
        }
        Ok(Value::Structure(structure))
    }

    fn collect_list(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        let mut list = List::new();
        while let Some(token) = tokens.peek() {
            if let Token::Col(Collection::ListEnd) = token {
                break;
            } else if let Token::Next = token {
                tokens.next();
            } else {
                list.append(self.collect_value(tokens)?.clone());
            }
        }
        Ok(Value::List(list))
    }

    fn expect(tokens: &mut Peekable<IntoIter<Token>>, token: Token) -> Result<()> {
        if let Some(token) = tokens.next() {
            Ok(())
        } else {
            Err(Error::Error("Expected target value"))
        }
    }
}