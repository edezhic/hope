mod build;
mod token;
mod translate;

pub use token::*;
use translate::Vocabulary;
use crate::core::*;

pub struct Bot {
    vocab: Vocabulary,
    terms: Structure,
    result: Value,
}

impl Bot {
    pub fn init() -> Result<Bot> {
        Ok(Bot {
            vocab: Vocabulary::english()?,
            terms: Structure::new(),
            result: Value::None,
        })
    }
    pub fn _do(&mut self, s: &str) -> Result<()> {
        let tokens = self.translate(Text::from_str(s))?;
        for token in tokens {
            println!("{:?}", token);
        }
        //let algorithm = self.build(tokens)?;
        //self.perform(tokens)
        Ok(())
    }
}