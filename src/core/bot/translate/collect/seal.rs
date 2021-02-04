use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_seal(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        let mut seal = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.literal_end(piece) {
                break;
            } else if self.vocab.whitespace(piece) {
                break;
            } else {
                seal.add(piece);
            }
        }
        tokens.push(Token::Val(Value::Seal(Seal::from_text(seal)?)));
        Ok(())
    }
}
