use crate::*;

fn context_lookup_script(term: &Text) -> Option<Id> {
    None
}
fn context_lookup_definition(term: &Text) -> Option<Text> {
    None
}
fn context_find_definition(term: &Text) -> Option<Text> {
    Some(term.clone())
}

pub fn build(indexed_tokens: Vec<IndexedToken>) -> Result<Builder> {
    let mut builder = Builder::init(indexed_tokens)?;

    while let Some((position, token)) = builder.iterate() {
        let (step, returns) = match token {
            Term(term) => {
                if let Some(id) = context_lookup_script(&term) {
                    unimplemented!("custom script handling in build")
                } else if let Some(id) = context_lookup_definition(&term) {
                    return Err(Message("Redifinitions aren't allowed?"));
                } else {
                    let definition = builder.add_node(Term(term));
                    builder.expect_token(Be)?;
                    builder.add_input_to(definition, Input, false)?;
                    // context.add_definition(term)
                    (definition, true)
                }
            }
            F(func) => {
                let syntax = func.syntax();
                let function = builder.add_node(F(func));
                if syntax.expects_input {
                    builder.add_input_to(function, Input, true)?;
                }
                for expected_prep in syntax.expected_args {
                    // prep/arg ordering?
                    let prep = builder.expect_token(P(expected_prep))?;
                    builder.add_input_to(function, prep, false)?;
                }
                (function, syntax.returns)
            }
            Dot | And => continue,
            Linebreak => {
                builder.this = None;
                continue;
            }
            C(If) => {
                let fork = builder.add_node(Or);
                //builder.add_conditions_to(fork)?;
                //let yes = builder.collect_statements_until(vec![C(Else), Dot, Linebreak]);
                //let no = builder.collect_statements_until(vec![Dot, Linebreak]);
                break //(vec![yes, no], false)
            }
            //P(For) => {
            // expect Each
            // expect Term
            // expect Mod
            // expect Term / collect input ?
            //C(Try) => {
            // add NodeIndex, remember it and keep going until ...? Break?
            _ => return Err(UnexpectedToken(token, position)), // Unexpected token?
        };
        builder.next_step(step, returns);
    }
    Ok(builder)
}

pub struct Builder {
    //pub namespace: HashMap<Id, &Builder>,
    pub tokens: IndexedTokenIter,
    pub graph: TokenGraph,
    pub syntax: Syntax,
    pub last_step: NodeIndex,
    pub this: Option<NodeIndex>,
}
impl Builder {
    pub fn init(indexed_tokens: Vec<IndexedToken>) -> Result<Builder> {
        let mut tokens = indexed_tokens.into_iter().peekable();

        let function = tokens.next().unwrap().1;
        if !function.of_type("Term") {
            return Err(Message("Expected a term for the function name"));
        }

        let mut graph = TokenGraph::new();
        let functionNode = graph.add_node(function);

        let mut builder = Builder {
            tokens,
            graph,
            last_step: functionNode,
            this: None,
            syntax: Syntax {
                expects_input: false,
                expected_args: vec![],
                returns: false,
            },
        };

        if let Term(term) = builder.peek_token()? {
            let input = builder.move_token_into_node();
            builder.link(input, functionNode, Input);
            builder.this = Some(input);
            builder.syntax.expects_input = true;
        }

        while builder.peek_until(Linebreak)? {
            let prep = builder.take_preposition()?;
            let arg = Term(builder.take_term()?);
            let argNode = builder.add_node(arg);
            builder.link(argNode, functionNode, P(prep.clone()));
            builder.syntax.expected_args.push(prep);
        }
        Ok(builder)
    }
    pub fn add_node(&mut self, token: Token) -> NodeIndex {
        self.graph.add_node(token)
    }
    pub fn move_token_into_node(&mut self) -> NodeIndex {
        let token = self.tokens.next().unwrap().1;
        self.graph.add_node(token)
    }
    pub fn link(&mut self, source: NodeIndex, target: NodeIndex, label: Token) {
        self.graph.add_edge(source, target, label);
    }
    pub fn add_conditions_to(&mut self, target: NodeIndex) -> Result<()> {
        while self.peek_until(C(Then))? {

        }
        Ok(())
    }
    pub fn add_input_to(&mut self, target: NodeIndex, label: Token, lookup_this: bool) -> Result<()> {
        let input = match self.peek_token()? {
            Term(term) if let Some(id) = context_lookup_script(term) => {
                unimplemented!("custom script handling in add_input_to")
            }
            Term(term) if let Some(id) = context_find_definition(term) => {
                self.move_token_into_node()
            }
            V(Struct(_)) => {
                let structure = self.move_token_into_node();
                while self.peek_until(CollectionEnd)? {
                    let attr_name = Term(Text::from_str("atrr")); // collect proper name for an attribute
                    self.add_input_to(structure, attr_name, false);
                }
                structure
            }
            V(Lst(_)) => {
                let list = self.move_token_into_node();
                while self.peek_until(CollectionEnd)? {
                    self.add_input_to(list, Input, false);
                }
                list
            }
            V(_) => self.move_token_into_node(),
            This => {
                if let Some(node) = self.this {
                    self.this.unwrap()
                } else {
                    return Err(Message(
                        "Unexpected local term: don't know where it should lead.",
                    ));
                }
            }
            //token if token.is_function() => ?
            //A(Start) => {collect formula}
            //Return => syntax.returns = true?
            // _ => try getting the last NodeIndex with syntax.returns_value == true
            _ => {
                if lookup_this && self.this.is_some() {
                    self.this.unwrap()
                } else {
                    return Err(Message("Unexpected token"));
                }
            }
        };
        Ok(self.link(input, target, label))
    }
    pub fn next_step(&mut self, next_step: NodeIndex, returns: bool) {
        self.link(self.last_step, next_step, Edge);
        self.last_step = next_step;
        if returns {
            self.this = Some(self.last_step);
        } else {
            self.this = None;
        }
    }
    pub fn take_token(&mut self) -> Result<Token> {
        if let Some((_, token)) = self.tokens.next() {
            Ok(token)
        } else {
            Err(Message("Expected token at the end of script"))
        }
    }
    pub fn take_preposition(&mut self) -> Result<Preposition> {
        if let P(preposition) = self.take_token()? {
            Ok(preposition)
        } else {
            Err(Message("Expected preposition"))
        }
    }
    pub fn take_term(&mut self) -> Result<Text> {
        if let Term(term) = self.take_token()? {
            Ok(term)
        } else {
            Err(Message("Expected term"))
        }
    }
    pub fn peek_token(&mut self) -> Result<&Token> {
        if let Some((_, token)) = self.tokens.peek() {
            Ok(token)
        } else {
            Err(Message("Expected token at the end of script"))
        }
    }
    pub fn iterate(&mut self) -> Option<(usize, Token)> {
        self.tokens.next()
    }
    pub fn expect_token(&mut self, token: Token) -> Result<Token> {
        if let Some((position, next_token)) = self.tokens.next() {
            if token == next_token {
                return Ok(token);
            }
            return Err(UnexpectedToken(next_token, position))
        }
        Err(ExpectedToken(token))
        
    }
    pub fn peek_until(&mut self, token: Token) -> Result<bool> {
        if *self.peek_token()? != token {
            Ok(true)
        } else {
            self.take_token();
            Ok(false)
        }
    }
}
