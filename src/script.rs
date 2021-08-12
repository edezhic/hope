use crate::*;
use crate::{Case::*, Flow::*, Modifier::*, Token::*};

// Script { Algorithm { Graph } }?
// Script { Graph }?

// Eval cases? Case => Cmd/Op/???
// Cases are somewhere between Cmds and Ops

// Add X to each of Y?
pub struct Algorithm {}

pub enum Node {
    Assignment,  // { Id }
    Instruction, // { command, eval-edges? }
    Control,     // { cases that include evals? Eval all cases? }
    Iterator,    // { collection, item-term-id? }
    Listener,    // { source, item-term-id? }
    Formula,     // ??? Ast?
}

pub fn build(vec: Vec<Token>) -> Result<Algorithm> {
    let mut tokens = Tokens::init(&vec);
    let mut algorithm = Algorithm {};
    while let Some(token) = tokens.peek {
        match token {
            Term(term) => {
                // Collect full Id/Ref
                if let Some(Being) = tokens.peek {
                    // Assigment (after evaluation of next)
                } else {
                    // Error
                }
            }
            Cmd(command) => {
                // Collect evaluations of arguments according to command.syntax()
            }
            F(If) => {
                // Collect cases until Then, then until Break or Else
                // If stops on break, check if Else is next
            }
            F(For) => {
                // Collect term, modifier and term/value
            }
            ListStart => {
                // Collect evals until ListEnd
            }
            StructStart => {
                // Collect either term+being+eval or plain terms
            }
            F(Break) | F(Then) | And => {
                // Skip when outside flow/case blocks
            }
            FormulaStart => {
                // Build AST inside graph?
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
