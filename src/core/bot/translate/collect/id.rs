use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_id(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        pieces.next();
        let mut id = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.whitespace.is_match(piece) {
                break;
            } else {
                id.add(piece);
            }
        }
        tokens.push(Token::Ref(Value::Id(Id::from_text(id)?)));
        Ok(())
    }
}