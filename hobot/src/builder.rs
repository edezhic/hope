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
/// How about:
///
/// collect -> pull? compose? ???
/// phrase -> instruction? ???

type PhraseOutput = (NodeIndex, bool);

pub fn build(indexed_tokens: Vec<IndexedToken>) -> Result<Builder> {
    let mut builder = Builder::init(indexed_tokens)?;

    while let Some(IndexedToken { index, token }) = builder.peek() {
        match token {
            If => {
                builder.skip(If);
                let fork = builder.attach_node(Or);
                builder.collect_conditions(fork)?;
                let remembered_this = builder.this;

                let then_branch = builder.collect_sentence(fork, V(Yes))?;
                let mut else_branch = fork;

                if builder.next_sentence_starts_with(Else)? {
                    builder.skip_until(Else);
                    builder.tip = fork;
                    builder.this = remembered_this;
                    else_branch = builder.collect_sentence(fork, V(No))?;
                }

                builder.merge(then_branch, else_branch);
                builder.this = None;
            }
            //P(For) => Each Term Prep(Of/In/?) Term collect_sentence ?
            //C(Try) => collect_sentence?
            Dot | And | Linebreak => {
                builder.skip_any();
            } // ?
            _ => {
                builder.attach_phrase(Edge)?;
            }
        }
    }
    Ok(builder)
}

pub struct Builder {
    iter: IndexedTokenIter,
    pub graph: TokenGraph,
    script_syntax: Syntax,
    tip: NodeIndex,
    this: Option<NodeIndex>, // if script contains Return (or this.is_some at the end?) then returns=true
}
impl Builder {
    pub fn init(indexed_tokens: Vec<IndexedToken>) -> Result<Builder> {
        let mut iter = indexed_tokens.into_iter().multipeek();

        let script = iter.next().unwrap().token;
        if !script.of_type("Term") {
            return Err(Message("Expected a term for the function name"));
        }

        let mut graph = TokenGraph::new();
        let scriptNode = graph.add_node(script);

        let mut builder = Builder {
            iter,
            graph,
            tip: scriptNode,
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

        while builder.loop_until(Linebreak)? {
            let prep = builder.take_preposition()?;
            let arg = Term(builder.take_term()?);
            let argNode = builder.add_node(arg);
            builder.link(argNode, scriptNode, P(prep));
            builder.script_syntax.expected_args.push(prep);
        }
        Ok(builder)
    }
    pub fn collect_sentence(&mut self, origin: NodeIndex, link: Token) -> Result<NodeIndex> {
        let mut tip = self.attach_phrase(link)?;
        while let Some(itoken) = self.peek() {
            if itoken == Linebreak || itoken == Dot || itoken == Else {
                break;
            }
            self.skip_optional(And);
            tip = self.attach_phrase(Edge)?;
        }
        Ok(tip)
    }
    pub fn attach_phrase(&mut self, link: Token) -> Result<NodeIndex> {
        let IndexedToken { index, token } = self.next().unwrap();
        let (phrase_node, returns) = match token {
            Term(term) => {
                if let Some(id) = context_lookup_script(&term) {
                    unimplemented!("custom script handling in attach_phrase")
                } else {
                    self.collect_definition(term)?
                }
            }
            F(function) => self.collect_function(function)?,
            //Return => self.script_syntax.returns = true,
            _ => return Err(UnexpectedPhraseToken(token, index)),
        };
        self.link(self.tip, phrase_node, link);
        self.tip = phrase_node;
        if returns {
            self.this = Some(self.tip);
        } else {
            self.this = None;
        }
        Ok(phrase_node)
    }

    pub fn collect_conditions(&mut self, target: NodeIndex) -> Result<()> {
        while let Some(IndexedToken { index, token }) = self.peek_until(Then)? {
            let mut comparison: Option<NodeIndex> = None;
            let left = self.collect_input(true)?;
            let mut right: Option<NodeIndex> = None;

            self.skip_optional(Be)?; // if Be => comparison = Equal?

            if let Not = self.peek_token()? {
                // 'No' link from comparison to target?
                return Err(Message("Unimplemented negation"));
            }
            if let C(_) = self.peek_token()? {
                let comparative = C(self.take_comparison()?);
                // if peek == Or => take more comparisons?
                comparison = Some(self.add_node(comparative));
                self.skip_optional(Than);
                // after all comparisons
                right = Some(self.collect_input(true)?);
            }

            if let Or = self.peek_token()? {
                // continue collecting conditions with the same target?
            } else if let And = self.peek_token()? {
                // add_node(And), change target to it and collect_conditions?
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
        self.skip(Then);
        Ok(())
    }
    pub fn collect_definition(&mut self, term: Text) -> Result<PhraseOutput> {
        let definition = self.add_node(Term(term));
        self.expect_token(Be)?;
        self.add_input_to(definition, Input, false)?;
        // context.add_definition(term)?
        Ok((definition, true))
    }
    pub fn collect_function(&mut self, function: Function) -> Result<PhraseOutput> {
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
    pub fn collect_input(&mut self, lookup_this: bool) -> Result<NodeIndex> {
        let IndexedToken { index, token } = self.next().unwrap();
        let input = match token {
            //Term(term) if let Some(id) = context_lookup_script(term) => {
            //    unimplemented!("custom script handling in add_input_to")
            //}
            Term(term) => self.add_node(Term(term)),
            V(Struct(structure)) => {
                let target = self.add_node(V(Struct(structure)));
                while self.loop_until(CollectionEnd)? {
                    let attr_name = Term(Text::from_str("atrr")); // collect proper name for an attribute
                    self.add_input_to(target, attr_name, false);
                }
                target
            }
            V(Lst(list)) => {
                let target = self.add_node(V(Lst(list)));
                while self.loop_until(CollectionEnd)? {
                    self.add_input_to(target, Input, false);
                }
                target
            }
            V(value) => self.add_node(V(value)),
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
                let (result, returns) = self.collect_function(function)?;
                if !returns {
                    return Err(FormattedMesssage(format!(
                        "Can't use function {:?} that doesn't return as input at {:?}",
                        self.graph[result], index
                    )));
                }
                result
            }
            //A(Start) => {collect formula}
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
                    return Err(UnexpectedInputToken(token, index));
                }
            }
        };
        Ok(input)
    }
    pub fn add_input_to(
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
    pub fn move_token_into_node(&mut self) -> NodeIndex {
        let token = self.next().unwrap().token;
        self.graph.add_node(token)
    }

    pub fn take_token(&mut self) -> Result<Token> {
        if let Some(IndexedToken { token, .. }) = self.next() {
            Ok(token)
        } else {
            Err(Message("Expected token at the end of script"))
        }
    }
    pub fn take_comparison(&mut self) -> Result<Comparative> {
        if let C(comparison) = self.take_token()? {
            Ok(comparison)
        } else {
            Err(Message("Expected comparison"))
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
    pub fn next_sentence_starts_with(&mut self, expected_token: Token) -> Result<bool> {
        self.iter.reset_peek();
        while let Some(itoken) = self.iter.peek() {
            if itoken == expected_token {
                return Ok(true);
            } else if itoken == Dot || itoken == Linebreak {
                continue;
            } else {
                break;
            }
        }
        Ok(false)
    }
    pub fn peek_token(&mut self) -> Result<&Token> {
        if let Some(IndexedToken { token, .. }) = self.peek() {
            Ok(token)
        } else {
            Err(Message("Expected token at the end of the script"))
        }
    }
    pub fn skip_optional(&mut self, token: Token) -> Result<()> {
        if self.peek_token()? == token {
            self.next();
        }
        Ok(())
    }
    pub fn skip_until(&mut self, token: Token) -> Result<()> {
        while self.peek_token()? != token {
            self.skip_any();
        }
        self.skip(token);
        Ok(())
    }
    pub fn skip(&mut self, token: Token) -> Result<()> {
        if self.peek_token()? == token {
            self.next();
            return Ok(());
        } else {
            return Err(ExpectedToken(token, 999));
        }
    }
    pub fn skip_any(&mut self) {
        self.next();
    }
    pub fn expect_token(&mut self, expected_token: Token) -> Result<Token> {
        if let Some(itoken) = self.next() {
            if itoken == expected_token {
                return Ok(expected_token);
            } else {
                return Err(ExpectedToken(expected_token, itoken.index));
            }
        }
        return Err(UnexpectedEnd);
    }
    pub fn peek_until(&mut self, end_token: Token) -> Result<Option<&IndexedToken>> {
        if let Some(itoken) = self.peek() {
            if itoken != end_token {
                Ok(Some(itoken))
            } else {
                Ok(None)
            }
        } else {
            Err(FormattedMesssage(format!(
                "Unexpected end, was looking for {:?}",
                end_token
            )))
        }
    }
    pub fn loop_until(&mut self, token: Token) -> Result<bool> {
        if self.peek_token()? != token {
            Ok(true)
        } else {
            self.next();
            Ok(false)
        }
    }
    pub fn next(&mut self) -> Option<IndexedToken> {
        self.iter.next()
    }
    pub fn peek(&mut self) -> Option<&IndexedToken> {
        self.iter.reset_peek();
        self.iter.peek()
    }
}
