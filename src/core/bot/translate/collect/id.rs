use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_id(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<Id> {
        pieces.next();
        let mut id = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.skip.is_match(piece) {
                break;
            } else {
                id.add(piece);
            }
        }
        Id::from_text(id)
    }
}
