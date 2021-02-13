mod collect;
mod pattern;
mod vocabulary;
use crate::core::*;
pub use pattern::*;
pub use vocabulary::*;

impl Bot {
    pub fn translate(&self, text: Text) -> Result<Vec<Token>> {
        let mut pieces = &mut text.split_by_word_bounds().peekable();
        let mut vec = Vec::<Token>::new();
        let mut tokens = &mut vec;

        while let Some(piece) = pieces.peek() {
            match Pattern::check(&self.vocab, piece) {
                Pattern::Ignore => { pieces.next(); }
                Pattern::Comment => self.collect_comment(pieces, tokens)?,
                Pattern::Id => self.collect_id(pieces, tokens)?,
                Pattern::Number => self.collect_number(pieces, tokens)?,
                Pattern::Seal => self.collect_seal(pieces, tokens)?,
                Pattern::Text => self.collect_text(pieces, tokens)?,
                Pattern::Time => self.collect_time(pieces, tokens)?,
                Pattern::Version => self.collect_version(pieces, tokens)?,
                Pattern::List => self.collect_list(pieces, tokens)?,
                Pattern::Struct => self.collect_structure(pieces, tokens)?,
                Pattern::Fact(fact) => {
                    tokens.push(fact);
                    pieces.next();
                }
                Pattern::Keyword(keyword) => {
                    tokens.push(keyword);
                    pieces.next();
                }
                Pattern::Command(command) => {
                    tokens.extend(command);
                    pieces.next();
                }
                Pattern::Result(result) => {
                    tokens.push(result);
                    pieces.next();
                },
                Pattern::Reference => self.collect_reference(pieces, tokens)?,
                Pattern::None => {
                    return Err(Error::ParsingError(format!(
                        r#"Unrecognized piece: '{}'"#,
                        piece
                    )));
                }
            }
        }
        Ok(vec)
    }
}
