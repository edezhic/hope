use crate::core::*;
use std::{iter::Peekable, vec::IntoIter};
use unicode_segmentation::UWordBounds;

impl Bot {
    pub fn collect_structure(&self, pieces: &mut Peekable<UWordBounds<'_>>) -> Result<Structure> {
        pieces.next();
        let mut structure = Structure::new();
        while let Some(piece) = pieces.peek() {
            if self.vocab.struct_end.is_match(piece) {
                pieces.next();
                break;
            }
            match self.read(pieces)? {
                Lexeme::Reference(value) => {
                    if let Value::Id(reference) = value {
                        let term = reference.get_term()?;
                        if self.vocab.op_define.is_match(pieces.peek().unwrap()) {
                            pieces.next();
                            match self.read(pieces)? {
                                Lexeme::Reference(value) | Lexeme::Item(value) => {
                                    structure.set(term, value)
                                }
                                lexeme => {
                                    return Err(Error::ParsingError(format!(
                                        r#"Unexpected structure attribute '{:?}'"#,
                                        lexeme
                                    )));
                                }
                            }
                        } else {
                            structure.set(term, Value::Id(reference));
                        } 
                    } else {
                        return Err(Error::ParsingError(format!(
                            r#"Unexpected structure attribute '{:?}'"#,
                            value
                        )));
                    }
                }
                lexeme => {
                    return Err(Error::ParsingError(format!(
                        r#"Unexpected structure lexeme '{:?}'"#,
                        lexeme
                    )));
                }
            }
        }
        Ok(structure)
    }
}
