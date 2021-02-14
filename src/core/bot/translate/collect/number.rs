use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_number(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<Number> {
        let number = pieces.next().unwrap();
        let sanitized = number.replacen(",", ".", 1);
        Number::from_string(sanitized)
    }
}
