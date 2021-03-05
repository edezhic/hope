use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_version(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<Version> {
        pieces.next();
        let mut version = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.skip.is_match(piece) {
                break;
            } else {
                version.add(piece);
            }
        }
        Version::from_text(version)
    }
}
