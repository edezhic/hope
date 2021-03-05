use crate::core::*;
use std::{iter::Peekable, vec::IntoIter};
use unicode_segmentation::UWordBounds;

impl Bot {
    pub fn collect_list(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<List> {
        pieces.next();
        let mut list = List::new();
        while let Some(piece) = pieces.peek() {
            if self.vocab.list_end.is_match(piece) {
                pieces.next();
                break;
            }
            match self.read(pieces)? {
                Token::Ref(value) => list.append(value),
                token => {
                    return Err(Error::ParsingError(format!(
                        r#"Unexpected list token '{:?}'"#,
                        token
                    )));
                }
            }
        }

        Ok(list)
    }
}
