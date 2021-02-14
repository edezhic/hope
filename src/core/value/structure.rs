use crate::core::*;
use std::{collections::HashMap, ops::Index};

type Key = Text;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Structure {
    content: HashMap<Key, Value>,
}

impl Structure {
    pub fn new() -> Structure {
        Structure {
            content: HashMap::new() 
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
