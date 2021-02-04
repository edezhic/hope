use super::Value;
use std::collections::VecDeque;
use derive_more::{IntoIterator};
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, IntoIterator)]
pub struct List {
    #[into_iterator(owned, ref, ref_mut)]
    values: VecDeque<Value>
}

impl List {
    pub fn new() -> List {
        List {
            values: VecDeque::new() 
        }
    }
    pub fn append(&mut self, value: Value) {
        self.values.push_back(value);
    }
    pub fn clear(&mut self) {
        self.values.clear();
    }
}