use std::vec::IntoIter;

use crate::core::*;

impl Bot {
    pub fn execute(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut iter = tokens.into_iter();
        let mut current_term: Option<Text> = None;
        let mut result: Option<Value> = None;

        while let Some(token) = iter.next() {
            //println!("---");
            //println!("{:?}", token);
            match token {
                Token::Term(term) => {
                    if !self.terms.contains(&term) {
                        self.terms.add(term.clone(), Value::default())
                    }
                    current_term = Some(term);
                }
                Token::Assign => {
                    result = Some(self.evaluate(&mut iter)?);
                    self.terms.set(current_term.take().unwrap(), result.take().unwrap());
                }
                Token::Cmd(command) => {
                    result = command.execute(self.evaluate(&mut iter)?)?;
                }
                Token::Mod(Modifier::New) => {}
                
                Token::Col(Collection::Start) => {
                    // value(collection) = self.collect?
                }
                Token::Val(value) => {
                    // this = value
                    //self.context.values.append(value)
                }
                Token::This => {
                    // self.context.values.last? self.context.result?
                }
                Token::Exp(Expression::Start) => {
                    // value = evaluate until exp::end
                }
                Token::Mod(Modifier::Next) => {}
                Token::Mod(Modifier::Binding) => {}
                Token::Mod(Modifier::Selection) => {}
                Token::Mod(Modifier::Targeting) => {}
                Token::Case(_) => {
                    // many different things
                }
                _ => ()
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, iter: &mut IntoIter<Token>) -> Result<Value> {
        if let Some(token) = iter.next() {
            if let Token::Val(value) = token {
                return Ok(value)
            } else if let Token::Term(term) = token {
                if self.terms.contains(&term) {
                    let value = self.terms.get(&term).unwrap().clone();
                    return Ok(value);
                } else {
                    return Err(Error::ExecutionError(format!(
                        r#"Unknown term: '{:?}'"#,
                        term
                    )));
                }
            } else {
                return Err(Error::ExecutionError(format!(
                    r#"Cannot evaluate: '{:?}'"#,
                    token
                )));
            }
        } else {
            return Err(Error::Error("Expected Value"));
        }
    }
}