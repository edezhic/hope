use crate::core::*;
use std::{collections::HashMap, ops::Index};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Structure {
    content: HashMap<Text, Value>,
}

impl Structure {
    pub fn new() -> Structure {
        Structure {
            content: HashMap::new() 
        }
    }
    pub fn get(&self, key: &Text) -> Option<&Value> {
        self.content.get(&key)
    }
    pub fn get_mut(&mut self, key: &Text) -> Option<&mut Value> {
        self.content.get_mut(&key)
    }

    pub fn contains(&self, key: &Text) -> bool {
        self.content.contains_key(key)
    }

    pub fn set(&mut self, key: Text, value: Value) {
        self.content.insert(key, value);
    }
}
