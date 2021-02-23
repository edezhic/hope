use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_reference(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<Id> {
        let mut selectors: Vec<Text> = vec![Text::lowercase(pieces.next().unwrap())];
        while let Some(piece) = pieces.peek() {
            if self.vocab.mod_none.is_match(piece) {
                pieces.next();
            } else if self.vocab.mod_s_of.is_match(piece) {
                pieces.next();
                // replace with "if self.read() == Lexeme::Ref"?
                let selector = pieces.find(|p| self.vocab.term.is_match(p)).unwrap();
                selectors.push(Text::lowercase(selector));
            } else {
                break;
            }
        }
        selectors.reverse();
        Ok(Id::reference(selectors))
    }
}
