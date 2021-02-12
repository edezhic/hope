use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_reference(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<bool> {
        //piece if self.result.is_match(piece) => Some(Token::Result),
        if self.vocab.term.is_match(piece) {
            Some(Token::Term(Text::lowercase(piece)))
        } else {
            None
        }
        // match term, result, selects, values?
        /*
        match tokens.next() {
            Some(Token::Result) => Ok(&self.result),
            Some(Token::Term(term)) => self.select(term, tokens),
            Some(Token::Val(value)) => {
                self.result.unsafe_set(value);
                Ok(&self.result)
            }
            Some(Token::Mod(Modifier::ListStart)) => {
                self.result.unsafe_set(self.collect_list(tokens)?);
                Ok(&self.result)
            }
            Some(Token::Mod(Modifier::StructStart)) => {
                self.result.unsafe_set(self.collect_struct(tokens)?);
                Ok(&self.result)
            }
            Some(Token::Mod(Modifier::ExpStart)) => {
                self.result.unsafe_set(self.evaluate(tokens)?);
                Ok(&self.result)
            }
            Some(token) => Err(Error::ExecutionError(format!(
                r#"I expected some reference or value, but found '{:?}'"#,
                token
            ))),
            None => Err(Error::Error(
                "I expected some reference or value, but found nothing",
            )),
        }*/
        /*
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
    }   */
        
        
    }
}
