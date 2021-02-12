use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_text(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        pieces.next();
        let mut text = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.val_text.is_match(piece) {
                break;
            } else {
                text.add(piece);
            }
        }
        tokens.push(Token::Ref(Value::Text(text)));
        Ok(())
    }
}