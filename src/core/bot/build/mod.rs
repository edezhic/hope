use std::{iter::Peekable, vec::IntoIter};
use crate::core::*;

impl Bot {
    pub fn build(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut tokens = &mut tokens.into_iter().peekable();
        Ok(())
    }
}