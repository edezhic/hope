use crate::*;
use petgraph::stable_graph::{StableDiGraph, NodeIndex};
use std::{vec::IntoIter, iter::Peekable};

pub struct Program {
    id: Id,
    args: Vec<(Modifier, Id)>,
    pub graph: StableDiGraph::<Token, Token>
}
impl Program {
    pub fn init(firstToken: Token) -> Result<Program> {
        if let Value(Value::Id(id)) = firstToken { // & is 1st-level ref
            Ok(Program {
                id,
                args: vec![],
                graph: StableDiGraph::<Token, Token>::new(),
            })
        } else {
            Err(Error::Error("Script name must come 1st"))
        }
    }
    pub fn add_arg(&mut self, modifier: Modifier, id: Id) {
        self.args.push((modifier, id))
    }
    pub fn assign(&mut self, target: Token) -> NodeIndex {
        let assigment = self.graph.add_node(Being);
        let target = self.graph.add_node(target);
        self.graph.add_edge(assigment, target, Cmd(Command::Send));
        assigment
    }
    pub fn add_value(&mut self, value: Token) -> NodeIndex {
        self.graph.add_node(value)
    }
    pub fn link(&mut self, source: NodeIndex, target: NodeIndex, label: Token) {
        self.graph.add_edge(source, target, label);
    }
}

pub struct TokensIterator {
    iter: Peekable<IntoIter<(usize, Token)>>
}
impl TokensIterator {
    pub fn init(tokens: Vec<(usize, Token)>) -> Result<TokensIterator> {
        if tokens.len() > 0 {
            Ok(TokensIterator { iter: tokens.into_iter().peekable() })
        } else {
            Err(Error::Error("Script cannot be empty"))
        }
    }
    pub fn take(&mut self) -> Token {
        self.iter.next().unwrap().1
    }
    pub fn take_mod(&mut self) -> Result<Modifier> {
        if let Token::Mod(modifier) = self.take() {
            Ok(modifier)
        } else {
            Err(Error::Error("Expected modifier"))
        }
    }
    pub fn take_id(&mut self) -> Result<Id> {
        if let Token::Value(Value::Id(id)) = self.take() { // && id.is_ref() { 
            Ok(id)
        } else {
            Err(Error::Error("Expected reference"))
        }
    }
    pub fn remain(&self) -> bool {
        if let Some(_) = self.iter.peek() {
            return true
        }
        false
    }
    pub fn peek(&self) -> Token {
        self.iter.peek().unwrap().1
    }
    pub fn skip(&mut self) {
        self.iter.next();
    }
}

pub fn link(vec: Vec<(usize, Token)>) -> Result<Program> {
    let mut tokens = TokensIterator::init(vec)?;
    let mut program = Program::init(tokens.take())?;
    
    if let Value(Value::Id(id)) = tokens.peek() {
        program.add_arg(Modifier::Input, id)
    }

    while tokens.peek() != Break {
        program.add_arg(tokens.take_mod()?, tokens.take_id()?);
    }
    tokens.skip();

    // parse headers of all scripts before parsing their bodies to construct namespace

    while tokens.remain() {
        //println!("{:?}", tokens.peek());
        match tokens.take() {
            token if token.is_ref() => {
                // check if ref leads to a script, if so - proceed like with Cmd, else:
                if tokens.peek() == Being { // replace with tokens.expect(Being)?
                    let assigment = program.assign(token); // hanging nodes
                    tokens.skip();
                    // COLLECT INPUT(!) / EVALUATE / ???: ------------------------------------------------
                    match tokens.peek() {
                        Value(_) => {
                            let value = program.add_value(tokens.take());
                            program.link(value, assigment, Cmd(Command::Get));
                        }
                        ListStart => { 
                            // create Append node for each collect_input sequentially? or
                            // create an aggr node and point each collect_input result there in parallel?
                            let list = graph.add_node(tokens.take());
                            while tokens.peek() != ListEnd {
                                // collect input (simplified?)
                                let list_item = graph.add_node(tokens.take());
                                graph.add_edge(list_item, list, Cmd(Command::Get));
                            }
                            tokens.skip();
                        }
                        StructStart => {
                            // collect struct
                        }
                        token if token.is_cmd() => {
                            // 
                        }
                        FormulaStart => {
                            // collect formula
                        }
                        _ => break // get Result as input?
                    }
                    // ------------------------------------------------------------------
                    graph.add_edge(last_node, assigment, Then);
                    //last_node = assigment;
                } 
                // else Unexpected token
            }
            token if token.is_cmd() => {
                // let syntax = token.syntax()
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
                //last_node = command;
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
    Ok(program)
}
