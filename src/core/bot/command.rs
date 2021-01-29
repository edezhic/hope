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
        //println!("executing {:?}", &self);
        for value in values.iter() {
            //println!("with value {:?}", value);
        }
        None
        // match?
    }
}
