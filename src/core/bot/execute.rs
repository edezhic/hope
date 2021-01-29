use crate::core::*;

pub struct Context {
    terms: Structure,
    values: List,
    result: Option<Value>,
    command: Option<Command>,
    // exp?
}

impl Context {
    pub fn new() -> Context {
        Context {
            terms: Structure::new(),
            values: List::new(),
            result: None,
            command: None,
        }
    }
    pub fn switch(&mut self) { // + shift?
        self.result = None;
        self.command = None;
        self.values.clear();
    }
}

impl Bot {
    pub fn interpret(&mut self, tokens: Vec<Token>) -> Result<Option<Value>> {
        let mut this: Value;
        for token in tokens.into_iter() {
            match token {
                Token::Cmd(command) => {
                    self.execute();
                    self.context.command = Some(command);
                }
                Token::Case(_) => {
                    // many different things
                }
                Token::Val(value) => {
                    self.context.values.append(value)
                }
                Token::Term(_) | Token::This => {
                    // 
                }
                Token::Exp(_) => {
                    // build a tree and evaluate until exp:end?
                }
                Token::Mod(_) => {
                    // ???
                }
                Token::Comment(_) => (), // nothing?
            }
        }
        self.execute()
    }

    pub fn execute(&mut self) -> Result<Option<Value>> {
        if let Some(cmd) = &self.context.command {
            if let Some(value) = cmd.execute(&self.context.values) {
                self.context.result = Some(value);
                self.context.values = List::new();
            }
        }
        Ok(None)
    }
}