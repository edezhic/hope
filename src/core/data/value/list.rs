use super::Value;
use std::collections::VecDeque;
use derive_more::{IntoIterator};
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, IntoIterator)]
pub struct List(VecDeque<Value>);

impl List {
    pub fn new() -> List {
        List(VecDeque::new())
    }
    pub fn append(&mut self, value: Value) {
        self.0.push_back(value);
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
}