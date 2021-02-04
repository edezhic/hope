use crate::core::*;
use std::{iter::Peekable, vec::IntoIter};

impl Bot {
    pub fn evaluate(&self, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value> {
        
        Ok(Value::default())
    }
}
