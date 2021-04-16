mod link;
mod token;
mod translate;

use crate::core::*;
pub use token::*;
pub use translate::*;

pub struct Bot {
    vocab: Vocabulary,
}

impl Bot {
    pub fn init() -> Result<Bot> {
        Ok(Bot {
            vocab: Vocabulary::english()?,
        })
    }
    pub fn _do(&self, s: &str) -> Result<()> {
        println!("{} ", s);
        let tokens = self.translate(s)?;
        print!("-----: ");
        for token in &tokens {
            print!("{} ", token);
        }
        println!("");
        let graph = self.link(tokens);
        Ok(())
    }
}
