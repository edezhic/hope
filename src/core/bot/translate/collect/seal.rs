use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_seal(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<Seal> {
        pieces.next();
        let mut seal = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.skip.is_match(piece) {
                break;
            } else {
                seal.add(piece);
            }
        }
        Seal::from_text(seal)
    }
}
