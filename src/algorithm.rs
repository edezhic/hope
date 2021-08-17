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
                // Collect full Ref(Id): N (of N (of N (of...)))
                if let Some(Being) = tokens.peek {
                    // Put assigment node after collecting expr
                } else {
                    // Unexpected token
                }
            }
            C(command) => {
                // Collect arguments' exprs according to command.syntax()
            }
            F(If) => {
                // Collect expr until Then, then statement until Break or Else
                // If stops on break, check if Else is next?
            }
            F(For) => {
                // Collect item name, specifier and expr, then statement
            }
            ListStart => {
                // Collect exprs until ListEnd
            }
            StructStart => {
                // Collect either name+being+expr or refs
            }
            F(Break) | F(Then) | And => {
                // Skip when outside flow/case blocks?
            }
            FormulaStart => {
                // Build AST node inside graph?
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