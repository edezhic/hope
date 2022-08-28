use petgraph::graph::Node;

use crate::*;

fn context_lookup_script(term: &Text) -> Option<Id> {
    None
}
fn context_lookup_definition(term: &Text) -> Option<Text> {
    // not in context?
    None
}
fn context_find_definition(term: &Text) -> Option<Text> {
    // not in context?
    Some(term.clone())
}

type PhraseResult = (NodeIndex, bool);

pub fn build(indexed_tokens: Vec<IndexedToken>) -> Result<Builder> {
    let mut builder = Builder::init(indexed_tokens)?;

    while let Some((position, token)) = builder.iterate() {
        let (phrase_result, returns) = match token {
            Term(term) => {
                if let Some(id) = context_lookup_script(&term) {
                    unimplemented!("custom script handling in build")
                } else if let Some(id) = context_lookup_definition(&term) {
                    return Err(Message("Redefinitions aren't allowed?"));
                } else {
                    builder.collect_definition(term)?
                }
            }
            F(function) => builder.collect_function(function)?,
            C(If) => {
                // Or node, conditions in, yes+no edges out
                break;
            }
            //P(For) => Each Term Prep(Of/In/?) Term collect_sentence ?
            //C(Try) => collect_sentence?
            Dot | And => continue,
            Linebreak => {
                builder.this = None; // ?
                continue;
            }
            _ => return Err(UnexpectedPhraseToken(token, position)),
        };
        builder.next_phrase(phrase_result, returns);
    }
    Ok(builder)
}

pub struct Builder {
    //pub namespace: HashMap<Id, &Builder>,
    tokens: IndexedTokenIter,
    pub graph: TokenGraph,
    script_syntax: Syntax,
    last_step: NodeIndex,
    this: Option<NodeIndex>, // if script contains C(Return) or this is_some at the end then returns=true
}
impl Builder {
    pub fn init(indexed_tokens: Vec<IndexedToken>) -> Result<Builder> {
        let mut tokens = indexed_tokens.into_iter().peekable();

        let script = tokens.next().unwrap().1;
        if !script.of_type("Term") {
            return Err(Message("Expected a term for the function name"));
        }

        let mut graph = TokenGraph::new();
        let scriptNode = graph.add_node(script);

        let mut builder = Builder {
            tokens,
            graph,
            last_step: scriptNode,
            this: None,
            script_syntax: Syntax {
                expects_input: false,
                expected_args: vec![],
                returns: false,
            },
        };

        if let Term(term) = builder.peek_token()? {
            let input = builder.move_token_into_node();
            builder.link(input, scriptNode, Input);
            builder.this = Some(input);
            builder.script_syntax.expects_input = true;
        }

        while builder.peek_until(Linebreak)? {
            let prep = builder.take_preposition()?;
            let arg = Term(builder.take_term()?);
            let argNode = builder.add_node(arg);
            builder.link(argNode, scriptNode, P(prep));
            builder.script_syntax.expected_args.push(prep);
        }
        Ok(builder)
    }
    pub fn next_phrase(&mut self, next_phrase: NodeIndex, returns: bool) {
        self.link(self.last_step, next_phrase, Edge);
        self.last_step = next_phrase;
        if returns {
            self.this = Some(self.last_step);
        } else {
            self.this = None;
        }
    }
    /*
    pub fn add_conditions_to(&mut self, target: NodeIndex) -> Result<()> {
        while self.peek_until(C(Then))? {

        }
        Ok(())
    }
     */
    pub fn collect_definition(&mut self, term: Text) -> Result<PhraseResult> {
        let definition = self.add_node(Term(term));
        self.expect_token(Be)?;
        self.add_input_to(definition, Input, false)?;
        // context.add_definition(term)
        Ok((definition, true))
    }
    pub fn collect_function(&mut self, function: Function) -> Result<PhraseResult> {
        let syntax = function.syntax();
        let target = self.add_node(F(function));
        if syntax.expects_input {
            self.add_input_to(target, Input, true)?;
        }
        for expected_prep in syntax.expected_args {
            // prep/arg ordering?
            let prep = self.expect_token(P(expected_prep))?;
            self.add_input_to(target, prep, false)?;
        }
        Ok((target, syntax.returns))
    }
    pub fn add_input_to(
        &mut self,
        target: NodeIndex,
        label: Token,
        lookup_this: bool,
    ) -> Result<()> {
        let (index, next_token) = self.iterate().unwrap();
        let input = match next_token {
            //Term(term) if let Some(id) = context_lookup_script(term) => {
            //    unimplemented!("custom script handling in add_input_to")
            //}
            Term(term) if let Some(id) = context_find_definition(&term) => self.add_node(Term(term)),
            V(Struct(structure)) => {
                let target = self.add_node(V(Struct(structure)));
                while self.peek_until(CollectionEnd)? {
                    let attr_name = Term(Text::from_str("atrr")); // collect proper name for an attribute
                    self.add_input_to(target, attr_name, false);
                }
                target
            }
            V(Lst(list)) => {
                let target = self.add_node(V(Lst(list)));
                while self.peek_until(CollectionEnd)? {
                    self.add_input_to(target, Input, false);
                }
                target
            }
            V(value) => self.add_node(V(value)),
            This => {
                if let Some(node) = self.this {
                    node
                } else {
                    return Err(FormattedMesssage(
                        format!("Unexpected local reference at {:?}: don't know where it should lead.", index)
                    ));
                }
            }
            F(function) =>  {
                let (result, returns) = self.collect_function(function)?;
                if !returns {
                    return Err(FormattedMesssage(
                        format!("Can't use function {:?} that doesn't return as input at {:?}", self.graph[result], index)
                    ))
                }
                result
            }
            //A(Start) => {collect formula}
            //Return => syntax.returns = true?
            token => {
                if lookup_this {
                    if let Some(node) = self.this {
                        node
                    } else {
                        return Err(FormattedMesssage(
                            format!("Expected local reference at {:?} but don't know where it should lead.", index)
                        ));
                    }
                }  else {
                    return Err(UnexpectedInputToken(token, index));
                }
            }
        };
        Ok(self.link(input, target, label))
    }
    pub fn add_node(&mut self, token: Token) -> NodeIndex {
        self.graph.add_node(token)
    }
    pub fn link(&mut self, source: NodeIndex, target: NodeIndex, label: Token) {
        self.graph.add_edge(source, target, label);
    }
    pub fn move_token_into_node(&mut self) -> NodeIndex {
        let token = self.tokens.next().unwrap().1;
        self.graph.add_node(token)
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
            Err(Message("Expected token at the end of the script"))
        }
    }
    pub fn iterate(&mut self) -> Option<(usize, Token)> {
        self.tokens.next()
    }
    pub fn expect_token(&mut self, token: Token) -> Result<Token> {
        if let Some((position, next_token)) = self.tokens.next() {
            if token == next_token {
                return Ok(token);
            } else {
                return Err(ExpectedToken(token, position));
            }
        }
        return Err(UnexpectedEnd);
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
