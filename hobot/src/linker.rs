use crate::{*, Value::*, Preposition::*, Algebra::*, Function::*, Control::*, Selector::*, Relation::*};
use petgraph::stable_graph::{StableDiGraph, NodeIndex as Node};
//use std::{vec::IntoIter, iter::Peekable};

pub struct Argument {
    prep: Option<Preposition>,
    id: Id
}
pub struct Program {
    id: Id,
    args: Vec<Argument>,
    pub graph: StableDiGraph::<Token, Token>
}
impl Program {
    pub fn init(firstToken: Token) -> Result<Program> {
        if let V(I(id)) = firstToken { // && is 1st-level ref
            Ok(Program {
                id,
                args: vec![],
                graph: StableDiGraph::<Token, Token>::new(),
            })
        } else {
            Err(Message("Script name must come 1st"))
        }
    }
    pub fn program_input(&mut self, id: Id) {
        self.args.push(Argument { prep: None, id })
    }
    pub fn program_arg(&mut self, prep: Preposition, id: Id) {
        self.args.push(Argument { prep: Some(prep), id })
    }
    pub fn add_input(&mut self, incoming: Token, target: Node, edge: Token) -> Node {
        let input = self.graph.add_node(incoming);
        self.graph.add_edge(input, target, edge);
        input
    }
    pub fn assign(&mut self, target: Token) -> Node {
        self.graph.add_node(target)
    }
    pub fn add_value(&mut self, value: Token) -> Node {
        self.graph.add_node(value)
    }
    pub fn link(&mut self, source: Node, target: Node, label: Token) {
        self.graph.add_edge(source, target, label);
    }
}


pub fn link(vec: Vec<(usize, Token)>) -> Result<Program> {
    let mut tokens = TokensIterator::init(vec)?;
    let mut program = Program::init(tokens.take())?; // first tokens until break (header)?
    
    if let V(I(id)) = tokens.peek() {
        program.program_input(tokens.take_id()?)
    }

    while *tokens.peek() != Newline {
        program.program_arg(tokens.take_prep()?, tokens.take_id()?);
    }
    tokens.next();

    // parse headers of all scripts before parsing their bodies to construct namespace?

    while tokens.remain() {
        match tokens.take() {
            token if token.is_ref() => {
                // check if ref leads to a script, if so - proceed like with Cmd, else:
                if *tokens.peek() == Be { // replace with tokens.expect(Be)?
                    let target = program.assign(token);
                    tokens.next();
                    // COLLECT INPUT(!) / EVALUATE / ???: ------------------------------------------------
                    match tokens.peek() {
                        V(Struct(_)) => {
                            // fill in struct
                        }
                        V(Lst(_)) => {
                            // create Append node for each collect_input sequentially? or
                            // create an aggr node and point each collect_input result there in parallel?
                            let list = program.add_input(tokens.take(), target, F(Get));
                            while *tokens.peek() != C(Closure) {
                                // collect input (simplified?)
                                program.add_input(tokens.take(), list, F(Get));
                            }
                            tokens.next();
                        }
                        V(_) => {
                            program.add_input(tokens.take(), target, F(Get)); 
                        }
                        token if token.is_function() => {
                            // 
                        }
                        A(Start) => {
                            // collect formula
                        }
                        _ => break // get Result as input?
                    }
                } 
                // else Unexpected token
            }
            token if token.is_function() => {
                // let syntax = token.syntax() --- STRICTLY ACCORDING TO IT
                /*
                let command = graph.add_node(token);
                match tokens.peek() {
                    token if token.is_ref() => {
                        let input = graph.add_node(token.clone());
                        graph.add_edge(input, command, Cmd(Command::Get));
                    }
                    _ => {
                        let result = graph.add_node(This);
                        graph.add_edge(result, command, Cmd(Command::Get));
                    }
                }
                graph.add_edge(last_node, command, Then);
                */
                //last_node = command;
                // expect CMD.modifier()?
                // collect argument
            }
            Dot | Newline | And => continue,
            C(If) => {
                // create Or node, input conditions until then and output yes+no edges
            } 
            P(For) => {
                // expect Each
                // expect Ref
                // expect Mod
                // expect Ref / collect input ?
            }
            C(Try) => {
                // add node, remember it and keep going until ...? Break?
            }
            _ => break // Unexpected token
        }

    } 
    Ok(program)
}
