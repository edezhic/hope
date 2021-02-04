mod perform;
mod token;
mod translate;

pub use token::*;
use translate::Vocabulary;
use crate::core::*;

pub struct Bot {
    vocab: Vocabulary,
    terms: Structure,
    result: Value,
    this: Value,
}

impl Bot {
    pub fn init() -> Result<Bot> {
        Ok(Bot {
            vocab: Vocabulary::english()?,
            terms: Structure::new(),
            result: Value::None,
            this: Value::None,
        })
    }
    pub fn _do(&mut self, s: &str) -> Result<()> {
        let tokens = self.translate(Text::from_str(s))?;
        self.perform(tokens)
    }
}