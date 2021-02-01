mod show;
mod sum;
use crate::core::*;
#[derive(Debug)]
pub enum Command {
    Await,
    Seal,
    Send,
    Show,
    Sum,
    Verify,
}

impl Command {
    pub fn execute(&self, value: Value) -> Result<Option<Value>> {
        match self {
            Command::Show => show::show(value),
            Command::Sum => sum::sum(value),
            _ => {
                return Err(Error::ExecutionError(format!(
                    r#"Command '{:?}' not implemented"#,
                    self
                )));
            }
        }
    }
}