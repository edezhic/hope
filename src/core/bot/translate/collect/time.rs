use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_time(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        let mut time = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.literal_end(piece) {
                break;
            } else if self.vocab.whitespace(piece) {
                break;
            } else {
                time.add(piece);
            }
        }
        tokens.push(Token::Val(Value::Time(Time::from_text(time)?)));
        Ok(())
    }
}