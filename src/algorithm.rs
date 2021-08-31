use crate::{Token::*, *};

// Graph of tokens? Some simple graph-like structure?
// Edge types: Default flow, Yes/No, iteration?

pub struct Syntax {
    //tokens: [Token] required (and optional ones? how to optional ones?)
// args(expressions) as V(None)? This for X?
// corresponding nodes also here?
}
impl Token {
    pub fn syntax(&self) -> Syntax {
        match self {
            N(_) => Syntax {
                // (Of N (Of N (Of N (...)))) Being Expr
                // No output?

                /*
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
                     */
            },
            C(Command::Add) => Syntax {
                // Expr1 (To Expr2) | (To Each Expr2?)
                // Output Expr2
                // 
            },
            C(Command::Show) => Syntax {
                // Expr
                // No output?
            },
            If => Syntax {
                // Expr (And/Or Expr(And/Or Expr (...))) Then Statement (Else Statement)
                // Then/Else statements might have outputs

                // Collect expr until Then, then statement until Break or Else
                // If stops on break, check if Else is next?

            },
            For => Syntax {
                // Each N In Expr Statement

                 // Collect item name????, specifier and expr, then statement
            },
            ListStart => Syntax {
                // Expr (Expr (Expr (...))) ListEnd
                // Output list (Collect Exprs?)
            },
            StructStart => Syntax {
                // N (Being Expr) (N (Being Expr) ...) StructEnd
                // Output struct
            },
            FormulaStart => Syntax {
                // hmm...
            },
            _ => todo!(),
        }
    }
}

pub fn build(s: &str) -> Result<()> {
    print_tokens(&Pieces::translate(s)?);
    let mut vec = Pieces::translate(s)?;
    let mut tokens = Tokens::init(&vec)?;
    while let Some(token) = tokens.peek {
        let syntax = token.syntax();
        tokens.next();
    }
    Ok(())
}
