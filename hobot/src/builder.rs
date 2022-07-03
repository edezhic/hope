use crate::*;

pub struct Program {
    //pub namespace: HashMap<Id, &Program>,
    pub graph: Graph<Token, Token>,
}
impl Program {
    pub fn init(id: Id) -> (Program, Node) {
        let mut graph = Graph::<Token, Token>::new();
        let script = graph.add_node(Script(id));
        (Program { graph }, script)
    }
    pub fn add_input(&mut self, input: Token, target: Node) {
        let incoming = self.graph.add_node(input);
        self.graph.add_edge(incoming, target, Input);
    }
    pub fn add_node(&mut self, token: Token) -> Node {
        self.graph.add_node(token)
    }
    pub fn link(&mut self, source: Node, target: Node, label: Token) {
        self.graph.add_edge(source, target, label);
    }
    pub fn collect_input(&mut self, tokens: &mut TokensIterator) -> Result<Node> {
        match tokens.peek() {
            token if token.is_ref() => Err(Message("NOT IMPLEMENTED collect is_ref values")),
            V(Struct(_)) => {
                // fill in struct
                let structure = self.add_node(tokens.take());
                while *tokens.peek() != CollectionEnd {
                    // collect input (simplified?)
                    self.add_input(tokens.take(), structure);
                }
                tokens.expect(CollectionEnd);
                Ok(structure)
            }
            V(Lst(_)) => {
                // create Append node for each collect_input sequentially? or
                // create an aggr node and point each collect_input result there in parallel?
                let list = self.add_node(tokens.take());
                while *tokens.peek() != CollectionEnd {
                    // collect input (simplified?)
                    self.add_input(tokens.take(), list);
                }
                tokens.expect(CollectionEnd);
                Ok(list)
            }
            V(_) => Ok(self.add_node(tokens.take())),
            //token if token.is_function() => {}
            //A(Start) => {collect formula}
            token => Err(UnexpectedToken(token.clone())),
        }
    }
}

pub fn build(vec: Vec<(usize, Token)>) -> Result<Program> {
    let mut tokens = TokensIterator::init(vec)?;
    let (mut program, script) = Program::init(tokens.take_ref()?);

    //println!("script {:?}", script);

    //println!("{:?}", Dot::with_config(program.graph, &[]));

    // --- Parsing Header (parse all headers before parsing their bodies to construct namespace?)
    program.add_input(V(I(tokens.take_ref()?)), script);

    while tokens.until(Linebreak) {
        let prep = tokens.take_prep()?;
        let arg = program.graph.add_node(V(I(tokens.take_ref()?)));
        program.link(arg, script, P(prep));
    }

    // --- Parsing Body
    let context_search_script = |token: &Token| -> Option<Id> { None };

    while let Some((_, token)) = tokens.next() {
        if let Some(id) = context_search_script(&token) {
            unimplemented!("script handling")
        } else if token.is_ref() {
            let variable = program.add_node(token);
            tokens.expect(Be);
            let input = program.collect_input(&mut tokens)?;
            program.link(input, variable, Input);
        } else if token.is_function() {
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
        //Dot | Linebreak | And => continue,
        //C(If) => {
        // create Or node, input conditions until then and output yes+no edges
        //P(For) => {
        // expect Each
        // expect Ref
        // expect Mod
        // expect Ref / collect input ?
        //C(Try) => {
        // add node, remember it and keep going until ...? Break?
        else {
            break; // Unexpected token
        }
    }
    Ok(program)
}
