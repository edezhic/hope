use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_time(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<Time> {
        pieces.next();
        let mut time = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.ignore.is_match(piece) {
                break;
            } else {
                time.add(piece);
            }
        }
        Time::from_text(time)
    }
}
