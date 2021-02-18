use super::Value;
use core::fmt;
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

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ");
        let length = self.values.len();
            if length > 0 {
            for index in (0..).take(length - 1) {
                write!(f, "{}, ", &self.values[index]);
            }
            if let Some(value) = self.values.get(length - 1) {
                write!(f, "{}", value);
            }
        }
        write!(f, " ]")
    }
}