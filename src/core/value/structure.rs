use crate::core::*;
use std::{collections::HashMap, ops::Index};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Structure(HashMap<Text, Value>);

impl Structure {
    pub fn new() -> Structure {
        Structure(HashMap::new())
    }
    pub fn get(&self, key: &Text) -> Option<&Value> {
        self.0.get(&key)
    }
    pub fn get_mut(&mut self, key: &Text) -> Option<&mut Value> {
        self.0.get_mut(&key)
    }

    pub fn contains(&self, key: &Text) -> bool {
        self.0.contains_key(key)
    }

    pub fn set(&mut self, key: Text, value: Value) {
        self.0.insert(key, value);
    }
}
