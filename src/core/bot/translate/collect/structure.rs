use crate::core::*;
use std::{iter::Peekable, vec::IntoIter};
use unicode_segmentation::UWordBounds;

impl Bot {
    pub fn collect_structure(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        let mut structure = Structure::new();
        while let Some(token) = tokens.peek() {
            if let Token::Mod(Modifier::StructEnd) = token {
                break;
            } else if let Some(Token::Term(term)) = tokens.next() {
                if let Some(Token::Mod(Modifier::Assign)) = tokens.peek() {
                    tokens.next();
                    structure.set(term, self.reference(tokens)?.clone());
                } else {
                    let value;
                    if self.terms.contains(&term) {
                        value = self.terms.get(&term).unwrap().clone();
                    } else {
                        value = Value::flag();
                    }
                    structure.set(term, value);
                }
            }
        }
        tokens.push(Token::Ref(Value::Structure(structure)));
        Ok(())
    }
}
