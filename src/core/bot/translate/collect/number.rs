use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_number(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        let piece = pieces.next().unwrap();
        let number = Number::from_string(piece.replacen(",", ".", 1))?;
        tokens.push(Token::Ref(Value::Number(number)));
        Ok(())
    }
}
