use crate::{*, Token::*};

pub struct Algorithm {}

pub enum Node {
    Assignment,  // { Id } (+ one input edge)
    Instruction, // { command } + inputs edges
    Iterator,    // { collection: Id, item: Id }
    Formula,     // ??? Ast?
}
// Edge types: Value, Yes/No, ...?

pub fn build(s: &str) -> Result<Algorithm> {
    print_tokens(&Pieces::translate(s)?);
    let mut vec = Pieces::translate(s)?;
    let mut tokens = Tokens::init(&vec)?;
    let mut algorithm = Algorithm {};
    while let Some(token) = tokens.peek {
        // Get token.syntax() for each token, even N or If, and go with it?
        let syntax = token.syntax();
        /*
        match token {
            N(name) => {
                // check if script? If not, ...
                // collect_ref { 
                let mut path = vec![name.clone()];
                tokens.next();
                while let Some(Of) = tokens.peek {
                    if let Some(N(name)) = tokens.next() {
                        path.push(name.clone());
                        tokens.next();
                    } else {
                        // Unexpected token
                    }
                }
                // }
                if let Some(Being) = tokens.peek {
                    // collect expression
                    // assign ref to expr
                } else {
                    // Unexpected token
                }
            }
            C(command) => {
                // Collect arguments' exprs according to command.syntax()
            }
            If => {
                // Collect expr until Then, then statement until Break or Else
                // If stops on break, check if Else is next?
            }
            For => {
                // Collect item name, specifier and expr, then statement
            }
            ListStart => {
                // Collect exprs until ListEnd
            }
            StructStart => {
                // Collect either name+being+expr or refs
            }
            Break | Then | And => {
                // Skip when outside flow/case blocks?
            }
            FormulaStart => {
                // Build AST node inside graph?
            }
            _ => {
                return Err(Error::ParsingError(format!(
                    r#"Unexpected token '{:#?}'"#,
                    token
                )))
            }
        }
         */
    }
    Ok(algorithm)
}
