mod collect;
mod vocabulary;
mod pattern;
use crate::core::*;
pub use pattern::*;
pub use vocabulary::*;

impl Bot {
    pub fn translate(&self, text: Text) -> Result<Vec<Token>> {
        let mut pieces = &mut text.split_by_word_bounds().peekable();
        let mut tokens = &mut Vec::<Token>::new();

        while let Some(piece) = pieces.peek() {
            match Pattern::check(&self.vocab, piece) {
                Pattern::Skip => continue,
                Pattern::Comment => self.collect_comment(pieces, tokens)?,
                Pattern::Id => self.collect_id(pieces, tokens)?,
                Pattern::Number => self.collect_number(pieces, tokens)?,
                Pattern::Seal => self.collect_seal(pieces, tokens)?,
                Pattern::Text => self.collect_text(pieces, tokens)?,
                Pattern::Time => self.collect_time(pieces, tokens)?,
                Pattern::Version => self.collect_version(pieces, tokens)?,
                Pattern::List => self.collect_list(pieces, tokens)?,
                Pattern::Struct => self.collect_structure(pieces, tokens)?,
                Pattern::None => {
                    if !self.collect_keyword(pieces, tokens)?
                        && !self.collect_reference(pieces, tokens)?
                    {
                        return Err(Error::ParsingError(format!(
                            r#"Unrecognized piece: '{}'"#,
                            piece
                        )));
                    }
                }
            }
        }
        Ok(*tokens)
    }
}
