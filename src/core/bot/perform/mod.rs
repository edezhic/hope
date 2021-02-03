mod collect;
mod commands;

use std::{iter::Peekable, vec::IntoIter};
use crate::core::*;

impl Bot {
    pub fn perform(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut tokens = &mut tokens.into_iter().peekable();
        let mut current_term: Option<Text> = None; // FIXME relative refs: result, this, it...

        while let Some(token) = tokens.next() {
            match token {
                Token::Term(term) => {
                    if !self.terms.contains(&term) {
                        self.terms.set(term.clone(), Value::flag())
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
                    // self.result = self.evaluate until exp::end ?
                }

                Token::Case(_) => {
                    // many different things
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn expect(tokens: &mut Peekable<IntoIter<Token>>, token: Token) -> Result<()> {
        if let Some(token) = tokens.next() {
            Ok(())
        } else {
            Err(Error::Error("Expected target value"))
        }
    }
}