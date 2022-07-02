use crate::*;
pub struct Program {
    pub graph: Graph<Token, Token>,
}
impl Program {
    pub fn init(id: Id) -> (Program, Node) {
        let mut graph = Graph::<Token, Token>::new();
        let script_id = graph.add_node(Script(id));
        (
            Program {
                graph: Graph::<Token, Token>::new(),
            },
            script_id,
        )
    }
    pub fn add_arg(&mut self, target: Node, edge: Token, input: Token) -> (Node, Node) {
        let incoming = self.graph.add_node(input);
        self.graph.add_edge(incoming, target, edge);
        (target, incoming)
    }
    pub fn add_input(&mut self, target: Node, input: Token) -> (Node, Node) {
        self.add_arg(target, Input, input)
    }
    pub fn add_node(&mut self, token: Token) -> Node {
        self.graph.add_node(token)
    }
    pub fn add_value(&mut self, value: Token) -> Node {
        self.graph.add_node(value)
    }
    pub fn link(&mut self, source: Node, target: Node, label: Token) {
        self.graph.add_edge(source, target, label);
    }
}

pub fn build(vec: Vec<(usize, Token)>) -> Result<Program> {
    let mut tokens = TokensIterator::init(vec)?;
    let (mut program, script) = Program::init(tokens.take_ref()?);

    // --- Parsing Header (parse all headers before parsing their bodies to construct namespace?)
    program.add_input(script, V(I(tokens.take_ref()?)));
    while *tokens.peek() != Linebreak {
        program.add_arg(script, P(tokens.take_prep()?), V(I(tokens.take_ref()?)));
    }
    tokens.next();

    // --- Parsing Body
    while tokens.remain() {
        match tokens.take() {
            token if token.is_ref() => {
                // check if ref leads to a script, if so - proceed like with Cmd, else:
                if *tokens.peek() == Be {
                    // replace with tokens.expect(Be)?
                    let variable = program.add_node(token);
                    tokens.next();
                    // COLLECT INPUT(!) / EVALUATE / ???: ------------------------------------------------
                    match tokens.peek() {
                        V(Struct(_)) => {
                            // fill in struct
                        }
                        V(Lst(_)) => {
                            // create Append node for each collect_input sequentially? or
                            // create an aggr node and point each collect_input result there in parallel?
                            let (list, _) = program.add_input(variable, tokens.take());
                            while *tokens.peek() != CollectionEnd {
                                // collect input (simplified?)
                                program.add_input(list, tokens.take());
                            }
                            tokens.next();
                        }
                        V(_) => {
                            program.add_input(variable, tokens.take());
                        }
                        token if token.is_function() => {
                            //
                        }
                        A(Start) => {
                            // collect formula
                        }
                        _ => break, // get Result as input?
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
            Dot | Linebreak | And => continue,
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
            _ => break, // Unexpected token
        }
    }
    Ok(program)
}
