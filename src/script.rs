use crate::*;
use crate::{Case::*, Flow::*, Modifier::*, Token::*};

pub struct Algorithm {
    // Graph?
}

pub enum Node {
    Assignment,  // { term }
    Instruction, // { command, args }
    Control,     // { cases }
    Iterator,    // { collection, item }
    Listener,    // { source, item }
    Formula,     // ???
}

pub enum Edge {
    // Default?
// Yes/No
// Value
}

pub fn build(vec: Vec<Token>) -> Result<Algorithm> {
    let mut tokens = Tokens::init(vec);
    let mut algorithm = Algorithm {};
    while let Some(token) = tokens.peek {
        match token {
            Term(term) => {
                match tokens.next() {
                    Some(Being) => {
                        // Assigment
                    }
                    Some(Mod(Selection)) => {
                        // Term (Value?)
                        // repeat
                        // Being
                    }
                    _ => {}
                }
            }
            Cmd(command) => {
                // Collect arguments
            }
            F(Break) => {
                // Skip
            }
            F(For) => {
                // Collect term, modifier and term/value
            }
            F(If) => {
                // Collect case by case until Then, then until Break or Else
            }
            F(Then) => {
                // skip when outside "if"?
            }
            And => {
                // skip when outside "if"?
            }
            FormulaStart => {
                // Build AST inside graph?
            }
            ListStart => {
                // Collect
            }
            StructStart => {
                // Collect
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

pub fn debug(s: &str) -> Result<()> {
    println!("{} ", s);
    let tokens = translate(s)?;
    print!("-----: ");
    print_tokens(&tokens);
    println!("");
    let algorithm = build(tokens)?;
    Ok(())
}

pub fn translate(s: &str) -> Result<Vec<Token>> {
    let text = Text::from_str(s);
    let mut pieces = Pieces::split(&text);
    let mut tokens = vec![];
    while let Some(piece) = pieces.peek {
        if let Some(value) = match_value(piece, &mut pieces)? {
            tokens.push(Val(value));
        } else if let Some(token) = match_token(piece) {
            tokens.push(token);
            pieces.next();
        } else if valid_term(piece) {
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
