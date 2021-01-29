use crate::core::*;
use std::{collections::HashMap, ops::Index};

#[derive(Debug)]
pub enum Model { // FIXME Drop it and infer from the structure of the structure?
    None,
    Table, // column1: Value::List, column2: Value::List, ...
    Tensor, // tensor: Value::List(Value::List(Value::List(...)))
    Graph, // nodes: Value::List(x, y, z), edges: Value::List(?)
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Structure(HashMap<Text, Value>);

impl Structure {
    pub fn new() -> Structure {
        Structure(HashMap::new())
    }
    pub fn get(&self, text: &Text) -> Option<&Value> {
        self.0.get(&text)
    }
    pub fn get_mut(&mut self, text: &Text) -> Option<&mut Value> {
        self.0.get_mut(&text)
    }
}
