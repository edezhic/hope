use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_version(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        pieces.next();
        let mut version = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.whitespace.is_match(piece) {
                break;
            } else {
                version.add(piece);
            }
        }
        tokens.push(Token::Ref(Value::Version(Version::from_text(version)?)));
        Ok(())
    }
}
