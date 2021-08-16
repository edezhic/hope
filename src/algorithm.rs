use crate::*;
use crate::{Flow::*, Specifier::*, Token::*};


pub struct Algorithm {}

pub enum Node {
    Assignment,  // { Id } + input edge
    Instruction, // { command (Id?) } + input edges (with specifiers?)
    Control,     // => Edges?
    Iterator,    // { collection: Id, item: Id } merge?
    Listener,    // { source: Id, item: Id }; merge these ^^^ ?
    Formula,     // ??? Ast?
}
// Edge types: Value, Yes/No, ...?


pub fn build(s: &str) -> Result<Algorithm> {
    print_tokens(&Pieces::translate(s)?);
    let mut vec = Pieces::translate(s)?;
    let mut tokens = Tokens::init(&vec)?;
    let mut algorithm = Algorithm {};
    while let Some(token) = tokens.peek {
        match token {
            N(name) => {
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
                // Collect name, modifier and name/value
            }
            ListStart => {
                // Collect evals until ListEnd
            }
            StructStart => {
                // Collect either name+being+eval or plain names
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