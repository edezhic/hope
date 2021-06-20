mod token;
mod vocabulary;

use crate::core::*;
pub use token::*;
pub use vocabulary::*;

use Token::{Term, Val};

pub struct Bot {
    vocab: Vocabulary,
}

impl Bot {
    pub fn init() -> Result<Bot> {
        Ok(Bot {
            vocab: Vocabulary::english()?,
        })
    }

    pub fn link(&self, tokens: Vec<Token>) -> Result<()> {
        let mut iter = tokens.iter().peekable();
        while let Some(token) = iter.next() {
            match token {
                Val(_) => (), // Check if in Expr?
                Term(_) => (), // expect Being?
                Token::O(_) => (), // Check Expr?
                Token::Cmd(_) => (), // Collect arguments
                Token::C(_) => (), // And/Or? Others only after Fs?
                Token::F(_) => (), // hmm... (if break and peek=break -> next)
                Token::Mod(_) => (), // Nonstarter
                Token::S(_) => (), // Collect and goto Val
                Token::Being => (), // Is this legit? Nonstarter?
                Token::This => (), // Same as Val
                // Collect necessary stuff and push into graph/algorithm?
                // Recursively repeat until end of script?

                // Algorithm node types: command, assign, iterate, check, collect?, expr?, then?
                // + all kinds of Fs
                // Insides depending on node type? Nodes inside nodes?
            }
        }
        Ok(())
    }

    pub fn debug(&self, s: &str) -> Result<()> {
        println!("{} ", s);
        let tokens = self.translate(s)?;
        print!("-----: ");
        print_tokens(&tokens);
        println!("");
        self.link(tokens);
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
