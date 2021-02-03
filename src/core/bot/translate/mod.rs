mod collect;
mod vocabulary;
pub use vocabulary::*;
use crate::core::*;

impl Bot {
    pub fn translate(&self, text: Text) -> Result<Vec<Token>> {
        let mut pieces = &mut text.split_by_word_bounds().peekable();
        let mut vec = Vec::<Token>::new();
        let mut tokens = &mut vec;

        while let Some(piece) = pieces.next() {
            match self.vocab.check_pattern(piece) {
                Pattern::Skip => continue,
                Pattern::Comment => self.collect_comment(pieces, tokens)?,
                Pattern::Expression => self.collect_expression(pieces, tokens)?,
                Pattern::Id => self.collect_id(pieces, tokens)?,
                Pattern::Number => self.collect_number(piece, tokens)?,
                Pattern::Seal => self.collect_seal(pieces, tokens)?,
                Pattern::Text => self.collect_text(pieces, tokens)?,
                Pattern::Time => self.collect_time(pieces, tokens)?,
                Pattern::Version => self.collect_version(pieces, tokens)?,
                Pattern::None => {
                    if let Some(t) = self.vocab.reserved(piece) {
                        tokens.push(t);
                    } else if let Some(t) = self.vocab.term(piece) {
                        tokens.push(t);
                    } else {
                        return Err(Error::ParsingError(format!(
                            r#"Unrecognized piece: '{}'"#,
                            piece
                        )));
                    }
                }
            }
        }
        Ok(vec)
    }
}
