mod execute;
mod scan;
mod token;
mod command;
mod vocabulary;
pub use token::*;
pub use vocabulary::*;
pub use execute::Context;
pub use command::Command;

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
        
        self.interpret(tokens); // FIXME -> compile/build
        // tokens -> Structure { code: List, refs: List, data: List }? 
        // Operational graph? Smth like Computational graph but a bit broader thing
        // then another method to run these graphs?
        Ok(())
    }
}