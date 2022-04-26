use crate::*;
use petgraph::graph::DiGraph;

pub fn link(tokens: Vec<(usize, Token)>) -> Result<DiGraph<Token, Token>> {
    // LINK TOKENS INTO GRAPH ----------------------------------------------
    let mut graph = DiGraph::<Token, Token>::new();
    let mut iter = tokens.into_iter().peekable();

    // Parse title ^^^^^^^^^^^^^^^^^^^^
    let script = graph.add_node(iter.next().unwrap().1);
    let input = graph.add_node(iter.next().unwrap().1);
    graph.add_edge(input, script, Token::Mod(Modifier::Input));
    
    while iter.peek().unwrap().1 != Break {
        let modifier = iter.next().unwrap().1;
        let arg = graph.add_node(iter.next().unwrap().1);
        graph.add_edge(arg, script, modifier);
    }
    
    iter.next();

    // parse headers of all scripts before parsing their bodies to construct namespace?

    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // Parse Body *********************

    let mut last_node = script;

    while let Some(_) = iter.peek() {
        //println!("{:?}", iter.peek().unwrap().1);
        match iter.next().unwrap().1 {
            token if token.is_ref() => {
                // if ref to script proceed like with Cmd

                if iter.peek().unwrap().1 == Being {
                    let assigment = graph.add_node(iter.next().unwrap().1);
                    let target = graph.add_node(token);
                    graph.add_edge(assigment, target, Cmd(Command::Store));
                    // collect input:
                    match iter.peek().unwrap().1 {
                        Value(_) => {
                            let value = graph.add_node(iter.next().unwrap().1);
                            graph.add_edge(value, assigment, Cmd(Command::Request));
                        }
                        ListStart => {
                            let list = graph.add_node(iter.next().unwrap().1);
                            while iter.peek().unwrap().1 != ListEnd {
                                // collect input (simplified?)
                                let list_item = graph.add_node(iter.next().unwrap().1);
                                graph.add_edge(list_item, list, Cmd(Command::Request));
                            }
                            iter.next();
                        }
                        _ => break // request result as input?
                    }
                    graph.add_edge(last_node, assigment, Then);
                    last_node = assigment;
                } 
                // else Unexpected token
            }
            token if token.is_cmd() => {
                let command = graph.add_node(token);
                match &iter.peek().unwrap().1 {
                    token if token.is_ref() => {
                        let input = graph.add_node(token.clone());
                        graph.add_edge(input, command, Cmd(Command::Request));
                    }
                    _ => {
                        let result = graph.add_node(This);
                        graph.add_edge(result, command, Cmd(Command::Request));
                    }
                }
                graph.add_edge(last_node, command, Then);
                last_node = command;
                // expect CMD.modifier()?
                // collect argument
            }
            Break | And => continue,
            If => {
                // collect conditions until then?
            } 
            For => {
                // expect Each
                // expect Ref
                // expect Mod
                // expect Ref / collect input ?
            }
            Where => {
                // filter? conditions?
            }
            Try => {
                // add node, remember it and keep going until ...? Break?
            }
            _ => break // Unexpected token
        }

    } 

    // ********************************

    Ok(graph)
}
