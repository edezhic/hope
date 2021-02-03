use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_number(&self, piece: &str, tokens: &mut Vec<Token>) -> Result<()> {
        let number = Number::from_string(piece.replacen(",", ".", 1))?;
        tokens.push(Token::Val(Value::Number(number)));
        Ok(())
    }
}