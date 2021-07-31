mod token;
mod vocabulary;

use crate::core::*;
pub use token::*;
pub use vocabulary::*;

use Token::*;

// Expect X at y ... where to end this block? 
// Replace with "for each X at Y"? Any reasons to get inputs other way? Open/close channels?
// Await? Gather? Collect?
// "For each X at Y" is the best option for now?

pub struct Algorithm {
    //inputs: Vec<Input>, // { channel: Id, jump: &Listener }
    code: Vec<Instruction>,
}

pub enum Instruction { // -> Block?
    Command, // -> Instruction? { command, args }
    Control, // -> Branch?
    Case, // => control? -> Check?
    Iterator,
    Listener,
    Expression,
    Assignment,

}

impl Bot {
    pub fn build(&self, tokens: Vec<Token>) -> Result<()> {
        let mut iter = tokens.into_iter().peekable();
        while let Some(token) = iter.next() {
            match token {
                Term(term) => { 
                    // expect Being or selection?
                }, 
                Token::Cmd(command) => { 
                    // Collect X and modifier+argument
                }, 
                Token::F(flow) => { 
                    // Much stuff (if break and peek=break -> next)
                }, 
                Token::S(set) => { 
                    // Collect and treat as Val
                }, 
                _ => return Err(Error::ParsingError(format!(
                    r#"Unexpected token '{}'"#,
                    token
                )))
            }
        }
        Ok(())
    }
}

pub struct Bot {
    vocab: Vocabulary,
}

impl Bot {
    pub fn init() -> Result<Bot> {
        Ok(Bot {
            vocab: Vocabulary::english()?,
        })
    }

    pub fn debug(&self, s: &str) -> Result<()> {
        println!("{} ", s);
        let tokens = self.translate(s)?;
        print!("-----: ");
        print_tokens(&tokens);
        println!("");
        self.build(tokens);
        Ok(())
    }

    pub fn translate(&self, s: &str) -> Result<Vec<Token>> {
        let text = Text::from_str(s);
        let mut pieces = Pieces::split(&text, &self.vocab);
        let mut tokens = vec![];
        while let Some(piece) = pieces.peek {
            if let Some(value) = self.vocab.match_value(piece, &mut pieces)? {
                tokens.push(Val(value));
            } else if let Some(token) = self.vocab.match_token(piece) {
                tokens.push(token);
                pieces.next();
            } else if self.vocab.valid_term(piece) {
                tokens.push(Term(Text::lowercase(piece)));
                pieces.next();
            } else {
                return Err(Error::ParsingError(format!(
                    r#"I don't know how to translate '{}'"#,
                    piece
                )));
            }
        }
        Ok(tokens)
    }
}
