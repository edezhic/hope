use super::Value;
use std::collections::VecDeque;
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct List(VecDeque<Value>);

impl List {
    pub fn new() -> List {
        List(VecDeque::new())
    }
    pub fn iter(&self) -> std::collections::vec_deque::Iter<Value> {
        self.0.iter()
    }
    pub fn append(&mut self, value: Value) {
        self.0.push_back(value);
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
}