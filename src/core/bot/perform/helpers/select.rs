use std::{iter::Peekable, vec::IntoIter};
use crate::core::*;

impl Bot {
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
}