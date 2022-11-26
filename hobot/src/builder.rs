use crate::*;

fn context_lookup_script(term: &Text) -> Option<Id> {
    None
}
/// Build loop logic:
/// * 'If' -> collect conditions, collect sentence(potentially multiple phrases), optional ['else' + another sentence]
/// * _    -> attach phrase(definition or function invocation)
/// Definitions and functions are collecting inputs which might be references, values or outputs of other fns

type PhraseOutput = (NodeIndex, bool);

pub fn build(indexed_tokens: Vec<IndexedToken>) -> Result<Builder> {
    let mut builder = Builder::init(indexed_tokens)?;

    while let Some(IndexedToken { index, token }) = builder.tokens.peek() {
        match token {
            If => {
                builder.tokens.skip(If);
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

                if builder.tokens.next_sentence_starts_with(Else)? {
                    builder.tokens.skip_until(Else);
                    builder.tip = fork;
                    builder.this = remembered_this;
                    else_branch = builder.collect_sentence(fork, V(No))?;
                }

                builder.merge(then_branch, else_branch);
                builder.this = None;
            }
            //C(Try) => collect_sentence?
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
            builder.link(argNode, scriptNode, P(prep));
            builder.script_syntax.expected_args.push(prep);
        }
        Ok(builder)
    }

    pub fn collect_sentence(&mut self, origin: NodeIndex, link: Token) -> Result<NodeIndex> {
        let mut tip = self.attach_phrase(link)?;
        while let Some(itoken) = self.tokens.peek() {
            if itoken == Linebreak || itoken == Dot || itoken == Else {
                break;
            }
            self.tokens.skip_optional(And);
            tip = self.attach_phrase(Edge)?;
        }
        Ok(tip)
    }

    pub fn collect_conditions(&mut self, target: NodeIndex) -> Result<()> {
        while let Some(IndexedToken { index, token }) = self.tokens.peek_until(Then)? {
            let mut comparison: Option<NodeIndex> = None;
            let left = self.collect_input(true)?;
            let mut right: Option<NodeIndex> = None;

            self.tokens.skip_optional(Be)?;

            if let Not = self.tokens.peek_token()? {
                // 'No' link from comparison to target?
                return Err(Message("Unimplemented negation"));
            }
            if let C(_) = self.tokens.peek_token()? {
                let comparative = C(self.tokens.take_comparison()?);
                // if peek == Or => take more comparisons?
                comparison = Some(self.add_node(comparative));
                self.tokens.skip_optional(Than);
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

            if let Some(node) = comparison {
                self.link(left, node, Input);
                self.link(right.unwrap(), node, Input);
                self.link(node, target, Input);
            } else {
                self.link(left, target, Input);
            }
            // Err(UnexpectedConditionToken(token, index))?
        }
        self.tokens.skip(Then);
        Ok(())
    }

    pub fn attach_phrase(&mut self, link: Token) -> Result<NodeIndex> {
        let IndexedToken { index, token } = self.tokens.peek().unwrap();
        let (phrase_tip, returns) = match token {
            // Term(term) if context_find_script(term) => ?
            F(_) => self.collect_function()?,
            _ if token.is_valid_ref_start() => (self.collect_definition()?, true), // Each X`s Y is ...? Wtf?
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

    pub fn collect_definition(&mut self) -> Result<NodeIndex> {
        let reference = self.collect_ref()?;
        self.tokens.expect(Be)?;
        self.attach_input_to(reference, Input, false)?;
        // context.add_definition(term)?
        Ok(reference)
    }

    pub fn collect_function(&mut self) -> Result<PhraseOutput> {
        let function = self.tokens.take_function()?;
        let syntax = function.syntax();
        let target = self.add_node(F(function.clone()));
        if syntax.expects_input {
            self.attach_input_to(target, Input, true)?;
        }
        // TODO: REPLACE LOOP WITH COLLECTING IN ANY ORDER, AND BREAK IF ENCOUNTERED A COMMA
        for expected_prep in syntax.expected_args {
            let prep = self.tokens.expect(P(expected_prep))?;
            self.attach_input_to(target, prep, false)?;
        }
        Ok((target, syntax.returns))
    }

    pub fn collect_ref(&mut self) -> Result<NodeIndex> {
        // TODO
        //Term(term) if let Some(id) = context_lookup_script(term) => ?

        // Terms, Possesives, Each, All, Where and other thingies
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
        Ok(reference)
    }

    pub fn collect_input(&mut self, lookup_this: bool) -> Result<NodeIndex> {
        let IndexedToken { index, token } = self.tokens.peek_Itoken()?;
        // the error ^ is wrong if fn at the end of the script takes `this` as input
        let input = match token {
            _ if token.is_valid_ref_start() => self.collect_ref()?,
            V(Struct(_)) => {
                let target = self.move_token_into_node()?;
                while self.tokens.next_isnt(CollectionEnd)? {
                    let attr_name = Term(Text::from_str("attr")); // collect proper name for an attribute
                    self.attach_input_to(target, attr_name, false);
                }
                target
            }
            V(Lst(_)) => {
                let target = self.move_token_into_node()?;
                while self.tokens.next_isnt(CollectionEnd)? {
                    self.attach_input_to(target, Input, false);
                }
                target
            }
            V(_) => self.move_token_into_node()?,
            This => {
                if let Some(node) = self.this {
                    node
                } else {
                    return Err(FormattedMesssage(format!(
                        "Unexpected local reference at {:?}: don't know where it should lead.",
                        index
                    )));
                }
            }
            F(function) => {
                if function.syntax().returns {
                    self.collect_function()?.0
                } else {
                    return Err(FormattedMesssage(format!(
                        "Can't use function {:?} that doesn't return as input to another thing at {:?}",
                        function.clone(), index
                    )));
                }
            }
            //A(Start) => { collect_formula }
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
                    return Err(UnexpectedInputToken(token.clone(), *index));
                }
            }
        };
        Ok(input)
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

    pub fn merge(&mut self, branch1: NodeIndex, branch2: NodeIndex) -> Result<()> {
        let node = self.add_node(Edge);
        self.link(branch1, node, Edge);
        self.link(branch2, node, Edge);
        self.tip = node;
        Ok(())
    }
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
    pub fn move_token_into_node(&mut self) -> Result<NodeIndex> {
        let token = self.tokens.next().unwrap().token;
        Ok(self.graph.add_node(token))
    }
}
