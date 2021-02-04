mod commands;
mod helpers;

use std::{iter::Peekable, vec::IntoIter};
use crate::core::*;

impl Bot {
    pub fn perform(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut tokens = &mut tokens.into_iter().peekable();

        while let Some(token) = tokens.next() {
            match token {
                Token::Term(term) => {
                    self.expect(tokens, Token::Assign)?;
                    let value = self.reference(tokens)?.clone();
                    self.terms.set(term, value);
                }
                Token::Cmd(command) => {
                    self.result = match command {
                        Command::Set => { 
                            let reference = self.reference(tokens)?;
                            self.expect(tokens, Token::Mod(Modifier::Targeting))?;
                            let value = self.reference(tokens)?.clone();
                            reference.unsafe_set(value);
                            Value::None
                        }
                        Command::Show => {
                            Command::show(self.reference(tokens)?)?
                        }
                        Command::Sum => {
                            Command::sum(self.reference(tokens)?)?
                        }
                        _ => {
                            return Err(Error::ExecutionError(format!(
                                r#"I don't know how to '{:?}'"#,
                                command
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
        Ok(())
    }
}