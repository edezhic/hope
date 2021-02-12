use std::{iter::Peekable, vec::IntoIter};
use crate::core::*;

impl Bot {
    pub fn build(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut tokens = &mut tokens.into_iter().peekable();
        /*
        while let Some(token) = tokens.next() {
            match token {
                Token::Ref(term) => {
                    self.expect(tokens, Token::Mod(Modifier::Assign))?;
                    let value = self.reference(tokens)?.clone();
                    self.terms.set(term, value);
                }
                Token::Op(op) => {
                    self.result = match op {
                        Op::Set => { 
                            let reference = self.reference(tokens)?;
                            self.expect(tokens, Token::Mod(Modifier::Targeting))?;
                            let value = self.reference(tokens)?.clone();
                            reference.unsafe_set(value);
                            Value::None
                        }
                        Op::Show => {
                            Op::show(self.reference(tokens)?)?
                        }
                        Op::Sum => {
                            Op::sum(self.reference(tokens)?)?
                        }
                        _ => {
                            return Err(Error::ExecutionError(format!(
                                r#"I don't know how to '{:?}'"#,
                                op
                            )));
                        }
                    }
                }

                Token::Case(_) => {
                    // many different things
                }
                _ => (),
            }
        }
        */
        Ok(())
    }
}