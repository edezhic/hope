use crate::core::*;
use std::{iter::Peekable, vec::IntoIter};
use unicode_segmentation::UWordBounds;

impl Bot {
    pub fn collect_list(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        pieces.next();
        /*
        let mut list = List::new();
        while let Some(token) = tokens.peek() {
            if let Token::Mod(Modifier::ListEnd) = token {
                break;
            } else {
                list.append(self.reference(tokens)?.clone());
            }
        }
        tokens.push(Token::Ref(Value::List(list)));
        */
        Ok(())
    }
}
