mod collect;
mod read;
mod vocabulary;

use crate::core::*;
pub use vocabulary::Vocabulary;

impl Bot {
    pub fn translate(&self, text: Text) -> Result<Vec<Token>> {
        let mut pieces = &mut text.split_by_word_bounds().peekable();
        let mut tokens = Vec::<Token>::new();
        while let Some(piece) = pieces.peek() {
            if let Ok(token) = self.read(pieces) {
                tokens.push(token)
            }
        }
        Ok(tokens)
    }
}
