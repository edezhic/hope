use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_seal(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        pieces.next();
        let mut seal = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.whitespace.is_match(piece) {
                break;
            } else {
                seal.add(piece);
            }
        }
        tokens.push(Token::Ref(Value::Seal(Seal::from_text(seal)?)));
        Ok(())
    }
}
