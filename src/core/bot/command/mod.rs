mod show;
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
    pub fn execute(&self, values: &List) -> Option<Value> {
        println!("Executing command {:?}", self);
        match self {
            Command::Await => { None }
            Command::Seal => { None }
            Command::Send => { None }
            Command::Show => {
                Command::show(values)
            }
            Command::Sum => { None }
            Command::Verify => { None }
        }
    }
}
