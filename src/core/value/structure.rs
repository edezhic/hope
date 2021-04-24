use crate::core::*;
use core::fmt;
use std::{collections::HashMap, iter::FromIterator, ops::Index};
type Key = Text;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Structure {
    content: HashMap<Key, Value>,
}

impl Structure {
    pub fn new() -> Structure {
        Structure {
            content: HashMap::new(),
        }
    }
    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.content.get(&key)
    }
    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        self.content.get_mut(&key)
    }

    pub fn contains(&self, key: &Key) -> bool {
        self.content.contains_key(key)
    }

    pub fn set(&mut self, key: Key, value: Value) {
        self.content.insert(key, value);
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ");
        let pairs = Vec::from_iter(self.content.iter());
        let length = pairs.len();
        if length > 0 {
            for index in (0..).take(length - 1) {
                write!(f, "{}: {}, ", pairs[index].0, pairs[index].1);
            }
            if let Some((key, value)) = pairs.get(length - 1) {
                write!(f, "{}: {}", key, value);
            }
        }

        write!(f, " }}")
    }
}
