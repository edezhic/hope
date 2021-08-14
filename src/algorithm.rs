use crate::*;
use crate::{Flow::*, Specifier::*, Token::*};

// Edge types: Value, Yes/No, ...?

// Term -> ? Word? Name?

pub struct Algorithm {}

pub enum Node {
    Assignment,  // { Id }
    Instruction, // { command, eval-edges? }
    Control,     // { cases that include evals? Eval all cases? }
    Iterator,    // { collection, item-term-id? }
    Listener,    // { source, item-term-id? }
    Formula,     // ??? Ast?
}

pub fn build(s: &str) -> Result<Algorithm> {
    let mut tokens = Tokens::init(s)?;
    let mut algorithm = Algorithm {};
    while let Some(token) = tokens.peek {
        match token {
            T(term) => {
                // Collect full Id/Ref
                if let Some(Being) = tokens.peek {
                    // Assigment (after evaluation of next)
                } else {
                    // Error
                }
            }
            C(command) => {
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