mod command;
mod execute;
mod token;
mod translate;
mod vocabulary;
pub use command::*;
pub use token::*;
pub use vocabulary::*;

use crate::core::*;

pub struct Bot {
    vocab: Vocabulary,
    terms: Structure,
    result: Option<Value>
}

impl Bot {
    pub fn init() -> Result<Bot> {
        Ok(Bot {
            vocab: Vocabulary::english()?,
            terms: Structure::new(),
            result: None
        })
    }
    pub fn perform(&mut self, s: &str) -> Result<()> {
        let task = self.translate(Text::from_str(s))?;
        self.execute(task)
    }
}