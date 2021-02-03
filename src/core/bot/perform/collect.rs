use std::{iter::Peekable, vec::IntoIter};
use crate::core::*;

impl Bot {
    pub fn collect_reference(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<&Value> {
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

    pub fn collect_value(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
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

    pub fn select(&self, term: Text, tokens: &mut Peekable<IntoIter<Token>>) -> Result<&Value> {
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

    pub fn collect_struct(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
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

    pub fn collect_list(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
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
}