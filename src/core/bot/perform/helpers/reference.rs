use crate::core::*;
use std::{iter::Peekable, vec::IntoIter};

impl Bot {
    pub fn reference(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<&Value> {
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
        }
    }
}
