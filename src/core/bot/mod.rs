mod token;
mod vocabulary;

use crate::core::*;
pub use token::*;
pub use vocabulary::*;

use Token::*;

pub struct Algorithm {
    // inputs?
// code: Vec<Block>? Vec<Scope> where Scope: Vec<Block>?
// Graph?
}

pub enum Node {
    Instruction, // { command, args }
    Control,     // { cases }
    Iterator,    // { collection, item }
    Listener,    // { source, item }
    Expression,  // AST?
    Assignment,  // { term, ? }
}

impl Bot {
    pub fn build(&self, tokens: Vec<Token>) -> Result<Algorithm> {
        let mut iter = tokens.into_iter().peekable();
        let mut algorithm = Algorithm {};
        while let Some(token) = iter.next() {
            match token {
                Term(term) => {
                    // Expect Being (or selection+term... and then Being)
                }
                Cmd(command) => {
                    // Collect arguments
                }
                F(flow) => {
                    // Much stuff (if break and peek=break -> next)
                }
                _ => {
                    return Err(Error::ParsingError(format!(
                        r#"Unexpected token '{}'"#,
                        token
                    )))
                }
            }
        }
        Ok(algorithm)
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
