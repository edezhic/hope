use crate::{Token::*, *};

// Edge types: Default flow, Yes/No, iteration?

type Syntax = [Modifier];

pub fn build(s: &str) -> Result<()> {
    print_tokens(&Pieces::translate(s)?);
    let mut vec = Pieces::translate(s)?;
    let mut tokens = Tokens::init(&vec)?;
    // Run some recursive function?
    while let Some((index, token)) = tokens.peek {
        match token {
            N(_) => {
                // NAMESPACE, check if script (or variable?)
                // Script X of Y: Script1 of a of Script2 of c of d 

                // (Of N (Of N (Of N (...)))) Being Expr
                // No output? Everything produces an output?
                // CASES. X is more than Y or w/e

                /*
                // ? If not, ...
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
                     */
            },
            C(Command::Add) => {
                // Expr1 (To Expr2) | (To Each Expr2?)
                // Output Expr2
                // 
            },
            C(Command::Show) => {
                // Expr
                // No output?
            },
            If => {
                // Expr (And/Or Expr(And/Or Expr (...))) Then Statement (Else Statement)
                // Then/Else statements might have outputs

                // Collect expr until Then, then statement until Break or Else
                // If stops on break, check if Else is next?

            },
            For => {
                // Each N In Expr Statement

                 // Collect item name????, specifier and expr, then statement
            },
            ListStart => {
                // Expr (Expr (Expr (...))) ListEnd
                // Output list (Collect Exprs?)
            },
            StructStart => {
                // N (Being Expr) (N (Being Expr) ...) StructEnd
                // Output struct
            },
            FormulaStart => {
                // hmm...
            },
            _ => todo!(),
        }
        tokens.next();
    }
    Ok(())
}
