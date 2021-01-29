use crate::core::*;
use super::*;

impl Command {
    pub fn show(values: &List) -> Option<Value> {
        for value in values.iter() {
            println!("showing value {:?}", value);
        }
        None
    }
}