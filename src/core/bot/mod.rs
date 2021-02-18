mod build;
mod token;
mod translate;

use crate::core::*;
pub use token::*;
pub use translate::*;

pub struct Bot {
    vocab: Vocabulary,
    terms: Structure,
    buffer: Option<Value>,
}

impl Bot {
    pub fn init() -> Result<Bot> {
        Ok(Bot {
            vocab: Vocabulary::english()?,
            terms: Structure::new(),
            buffer: None,
        })
    }
    pub fn _do(&mut self, s: &str) -> Result<()> {
        println!("{} ", s);
        let tokens = self.translate(Text::from_str(s))?;
        for token in tokens {
            print!("{} ", token);
        }
        //let algorithm = self.build(tokens)?;
        //self.perform(tokens)
        Ok(())
    }
}
