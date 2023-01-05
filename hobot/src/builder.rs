use crate::*;

fn context_lookup_script(term: &Text) -> Option<Id> {
    None
}

type PhraseOutput = (NodeIndex, bool);

/// Build loop logic:
/// * 'If'  -> collect conditions, collect sentence(potentially multiple phrases), optional ['else' + another sentence]
/// *  _    -> attach phrase(definition or command invocation)
/// * 'Try' -> TODO
/// Definitions and commands are collecting inputs which might be references, values or outputs of other fns
pub fn build(indexed_tokens: Vec<IndexedToken>) -> Result<Builder> {
    let mut builder = Builder::init(indexed_tokens)?;

    while let Some(IndexedToken { index, token }) = builder.tokens.peek() {
        match token {
            F(If) => {
                builder.tokens.skip(F(If));
                let fork = match builder.tokens.peek_token()? {
                    Either => {
                        builder.tokens.skip(Either);
                        builder.attach_node(Or)
                    }
                    _ => builder.attach_node(And),
                };

                builder.collect_conditions(fork)?;

                let remembered_this = builder.this;

                let then_branch = builder.collect_sentence(fork, V(Yes))?;
                let mut else_branch = fork;

                if builder.tokens.next_sentence_starts_with(F(Else))? {
                    builder.tokens.skip_until(F(Else));
                    builder.tip = fork;
                    builder.this = remembered_this;
                    else_branch = builder.collect_sentence(fork, V(No))?;
                }

                builder.merge(then_branch, else_branch);
                builder.this = None;
            }
            Dot | And | Linebreak => {
                builder.tokens.skip_any();
            } // ?
            _ => {
                builder.attach_phrase(Edge)?;
            }
        }
    }
    Ok(builder)
}

pub struct Builder {
    tokens: IndexedTokensIter,
    pub graph: TokenGraph,
    script_syntax: Syntax, // if script contains Return (or this.is_some at the end?) then returns=true
    tip: NodeIndex,
    this: Option<NodeIndex>,
}
impl Builder {
    pub fn init(indexed_tokens: Vec<IndexedToken>) -> Result<Builder> {
        let mut iter = IndexedTokensIter::init(indexed_tokens);

        let script = iter.take_term()?;
        let mut graph = TokenGraph::new();
        let scriptNode = graph.add_node(Term(script));

        let mut builder = Builder {
            tokens: iter,
            graph,
            tip: scriptNode,
            this: None,
            script_syntax: Syntax {
                expects_input: false,
                expected_args: vec![],
                returns: false,
            },
        };

        if let Term(term) = builder.tokens.peek_token()? {
            let input = builder.move_token_into_node()?;
            builder.link(input, scriptNode, Input);
            builder.this = Some(input);
            builder.script_syntax.expects_input = true;
        }

        while builder.tokens.next_isnt(Linebreak)? {
            let prep = builder.tokens.take_preposition()?;
            let arg = Term(builder.tokens.take_term()?);
            let argNode = builder.add_node(arg);
            builder.link(argNode, scriptNode, S(prep));
            builder.script_syntax.expected_args.push(prep);
        }
        Ok(builder)
    }

    /// Attaches phrases until sentence breaker: Linebreak, Dot, F(Else), ?
    pub fn collect_sentence(&mut self, origin: NodeIndex, link: Token) -> Result<NodeIndex> {
        let mut tip = self.attach_phrase(link)?;
        while let Some(itoken) = self.tokens.peek() {
            if itoken == Linebreak || itoken == Dot || itoken == F(Else) {
                break;
            }
            self.tokens.skip_optional(And);
            tip = self.attach_phrase(Edge)?;
        }
        Ok(tip)
    }

    /// Collect left + relation + right parts until F(Then)
    pub fn collect_conditions(&mut self, target: NodeIndex) -> Result<()> {
        while let Some(IndexedToken { index, token }) = self.tokens.peek_until(F(Then))? {
            let mut relation: Option<NodeIndex> = None;
            let left = self.collect_input(true)?;
            let mut right: Option<NodeIndex> = None;

            self.tokens.skip_optional(Be)?;

            if let R(_) = self.tokens.peek_token()? {
                let relationship = R(self.tokens.take_relationship()?);
                // if peek == Or => take more comparisons?
                relation = Some(self.add_node(relationship));
                self.tokens.skip_optional(R(Than));
                // after all comparisons
                right = Some(self.collect_input(true)?);
            }

            if let Or = self.tokens.peek_token()? {
                // continue collecting conditions with the same target?
                return Err(Message("Unimplemented OR conditions"));
            } else if let And = self.tokens.peek_token()? {
                // add_node(And), change target to it and collect_conditions?
                return Err(Message("Unimplemented AND conditions"));
            }

            if let Some(node) = relation {
                self.link(left, node, Input);
                self.link(right.unwrap(), node, Input);
                self.link(node, target, Input);
            } else {
                self.link(left, target, Input);
            }
            // Err(UnexpectedConditionToken(token, index))?
        }
        self.tokens.skip(F(Then));
        Ok(())
    }

    /// Attach command or definition to link
    pub fn attach_phrase(&mut self, link: Token) -> Result<NodeIndex> {
        let IndexedToken { index, token } = self.tokens.peek().unwrap();
        let (phrase_tip, returns) = match token {
            C(_) => self.collect_command()?,
            Term(_) | S(_) => (self.collect_definition()?, true), // Each X`s Y is ...? Wtf?
            //Return => self.script_syntax.returns = true, collect_input,
            _ => return Err(UnexpectedPhraseToken(token.clone(), *index)),
        };
        self.link(self.tip, phrase_tip, link);
        self.tip = phrase_tip;
        if returns {
            self.this = Some(self.tip);
        } else {
            self.this = None;
        }
        Ok(phrase_tip)
    }

    /// Collect ref and attach input to it
    pub fn collect_definition(&mut self) -> Result<NodeIndex> {
        let (reference, returns) = self.collect_ref()?;
        // what if returns vs !returns?
        self.tokens.expect(Be)?;
        self.attach_input_to(reference, Input, false)?;
        // context.add_definition(term)?
        Ok(reference)
    }

    /// Collect command with arguments
    pub fn collect_command(&mut self) -> Result<PhraseOutput> {
        let command = self.tokens.take_command()?;
        let syntax = command.syntax();
        let target = self.add_node(C(command.clone()));
        if syntax.expects_input {
            self.attach_input_to(target, Input, true)?;
        }
        // TODO: REPLACE LOOP WITH COLLECTING IN ANY ORDER, AND BREAK IF ENCOUNTERED A COMMA OR FOUND ALL EXPECTED ARGS
        for expected_prep in syntax.expected_args {
            let prep = self.tokens.expect(S(expected_prep))?;
            self.attach_input_to(target, prep, false)?;
        }
        Ok((target, syntax.returns))
    }

    pub fn collect_ref(&mut self) -> Result<PhraseOutput> {
        // X's Y             ||| <- T(Y) <-S(P)- T(X) 
        // Each X            ||| <- T(X)><S(Each) 
        // Each X's Y        ||| <- T(Y)><S(Each) <-S(P)- T(X) 
        // Each X's any Y    ||| <- T(Y)><S(Any) <-S(P)- T(X)><S(Each) 
        // X's each Y        ||| <- T(Y)><S(Each) <-S(P)- T(X) 
        // Each X that C     ||| <- T(X)><S(Each) <-S(That)- C
        // X's any Y where C ||| <- T(Y)><S(Any) <-S(P)- T(X)
        // ---               |||       ^-S(Where)- C
        // X such that C     ||| <- T(X) <-S(That)- C
        // Any X that C      ||| <- T(X)><S(Any) <-S(That)- C
        while let Some(IndexedToken { index, token }) = self.tokens.peek() {
            match token {
                Term(_) => {} // check wtf, add node anyway
                Possessive => {} // link it next to current
                That => {} // collect conditions and link to current with That
                S(Each) | S(Any) | S(All) => {} // link next to itself?
                S(_) => {} // arguments?
                _ => {} // ??????
            }
        }

        // OLD STUFF BELOW
        let mut reference = self.move_token_into_node()?;
        
        while let Some(IndexedToken {
            token: Possessive, ..
        }) = self.tokens.peek()
        {
            self.tokens.skip(Possessive);
            let subterm = Term(self.tokens.take_term()?);
            let subterm_node = self.add_node(subterm);
            self.link(reference, subterm_node, Possessive);
            reference = subterm_node;
        }
        Ok((reference, true))
        // /OLD STUFF
    }

    /// Collect anything evaluable 
    pub fn collect_input(&mut self, lookup_this: bool) -> Result<NodeIndex> {
        let IndexedToken { index: idx, token } = self.tokens.peek_Itoken()?;
        let index = idx.clone();
        // the error ^ is wrong if fn at the end of the script takes `this` as input
        let input = match token {
            V(_) => self.collect_value()?,
            This => {
                let Some(node) = self.this else {
                    return Err(FormattedMesssage(format!(
                        "Unexpected local reference at {:?}: don't know where it should lead.",
                        index
                    )));
                    
                };
                node
            }
            Term(_) | S(_)  => {
                let (reference, returns) = self.collect_ref()?;
                if returns {
                    reference
                } else {
                    return Err(FormattedMesssage(format!(
                        "Expected input at {:?} but reference doesn't return any",
                        index
                    )));
                }
            }
            C(command) => {
                if command.syntax().returns {
                    self.collect_command()?.0
                } else {
                    return Err(FormattedMesssage(format!(
                        "Can't use command {:?} that doesn't return as input to another thing at {:?}",
                        command.clone(), index
                    )));
                }
            }
            //A(Start) => { collect_formula }
            // any other token
            token => {
                if lookup_this {
                    if let Some(node) = self.this {
                        self.this.take().unwrap()
                    } else {
                        return Err(FormattedMesssage(format!(
                            "Expected local reference at {:?} but don't know where it should lead.",
                            index
                        )));
                    }
                } else {
                    return Err(UnexpectedInputToken(token.clone(), index));
                }
            }
        };
        Ok(input)
    }

    pub fn collect_value(&mut self) -> Result<NodeIndex> {
        match self.tokens.peek_token()? {
            V(Struct(_)) => {
                let target = self.move_token_into_node()?;
                while self.tokens.next_isnt(CollectionEnd)? {
                    let attr_name = Term(Text::from_str("attr")); 
                    // collect proper name for an attribute ^^^
                    self.attach_input_to(target, attr_name, false);
                }
                Ok(target)
            }
            V(Lst(_)) => {
                let target = self.move_token_into_node()?;
                while self.tokens.next_isnt(CollectionEnd)? {
                    self.attach_input_to(target, Input, false);
                }
                Ok(target)
            }
            V(_) => Ok(self.move_token_into_node()?),
            _ => unreachable!("shouldn't be reachable?"),
        }
    }

    pub fn attach_input_to(
        &mut self,
        target: NodeIndex,
        label: Token,
        lookup_this: bool,
    ) -> Result<()> {
        let input = self.collect_input(lookup_this)?;
        Ok(self.link(input, target, label))
    }

    /// Connect two nodes with Edges into Edge node
    pub fn merge(&mut self, branch1: NodeIndex, branch2: NodeIndex) -> Result<()> {
        let node = self.add_node(Edge);
        self.link(branch1, node, Edge);
        self.link(branch2, node, Edge);
        self.tip = node;
        Ok(())
    }

    /// Append current tip with a node
    pub fn attach_node(&mut self, token: Token) -> NodeIndex {
        let node = self.add_node(token);
        self.link(self.tip, node, Edge);
        self.tip = node;
        node
    }
    pub fn add_node(&mut self, token: Token) -> NodeIndex {
        self.graph.add_node(token)
    }
    pub fn link(&mut self, source: NodeIndex, target: NodeIndex, label: Token) {
        self.graph.add_edge(source, target, label);
    }
    /// Unwrap next token and add to the graph
    pub fn move_token_into_node(&mut self) -> Result<NodeIndex> {
        let token = self.tokens.next().unwrap().token;
        Ok(self.graph.add_node(token))
    }
}
