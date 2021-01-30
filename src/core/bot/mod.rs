mod build;
mod scan;
mod token;
mod vocabulary;
pub use token::*;
pub use vocabulary::*;
pub use build::Context;

use crate::core::*;

pub struct Bot {
    vocab: Vocabulary,
    context: Context,
}

impl Bot {
    pub fn init() -> Result<Bot> {
        Ok(Bot {
            vocab: Vocabulary::english()?,
            context: Context::new(),
        })
    }
    pub fn perform(&mut self, s: &str) -> Result<()> {
        let tokens = self.scan(Text::from_str(s))?;
        
        println!("Tokens: ");
        for token in tokens.iter() {
            println!("{:?}", token);
        }
        
        let program = self.build(tokens);
        Ok(())
    }
}