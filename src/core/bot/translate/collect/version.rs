use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_version(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        let mut version = Text::empty();
        while let Some(piece) = pieces.next() {
            if let Some(end) = self.vocab.literal_end(piece) {
                tokens.push(end);
                break;
            } else if self.vocab.whitespace(piece) {
                break;
            } else {
                version.add(piece);
            }
        }
        tokens.push(Token::Val(Value::Version(Version::from_text(version)?)));
        Ok(())
    }
}
