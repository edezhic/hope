use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_reference(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        let mut selectors: Vec<Text> = vec![Text::lowercase(pieces.next().unwrap())];
        while let Some(piece) = pieces.peek() {
            if self.vocab.ignore.is_match(piece) {
                pieces.next();
            } else if self.vocab.mod_selection.is_match(piece) {
                pieces.next();
                let selector = pieces.find(|p| self.vocab.term.is_match(p)).unwrap();
                selectors.push(Text::lowercase(selector));
            } else {
                break;
            }
        }

        let term = selectors.pop();
        selectors.reverse();

        if selectors.len() > 0 {
            tokens.push(Token::Ref(Value::Id(Id::reference(term, Some(selectors)))));
        } else {
            tokens.push(Token::Ref(Value::Id(Id::reference(term, None))));
        }
        Ok(())
    }
}
