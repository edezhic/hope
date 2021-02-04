use super::*;
use crate::core::Result;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::{UWordBounds, UnicodeSegmentation};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Text(String);
impl Text {
    pub fn new(s: Option<&str>) -> Text {
        if let Some(value) = s {
            Text(Text::norm(value.to_owned()))
        } else {
            Text("".to_owned())
        }
    }
    pub fn empty() -> Text {
        Text("".to_owned())
    }
    pub fn add(&mut self, s: &str) {
        self.0.push_str(&Text::norm(s.to_owned()));
    }
    pub fn from_string(string: String) -> Text {
        Text::new(Some(&string))
    }
    pub fn from_str(s: &str) -> Text {
        Text::new(Some(s))
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn lowercase(s: &str) -> Text {
        Text::new(Some(&s.to_lowercase()))
    }
    pub fn split_by_word_bounds(&self) -> UWordBounds {
        UnicodeSegmentation::split_word_bounds(self.as_str())
    }
    fn norm(s: String) -> String {
        s.nfc().collect::<String>() // Canonical Normalization
    }
}

impl From<&str> for Text {
    fn from(item: &str) -> Self {
        Text::from_str(item)
    }
}

impl From<String> for Text {
    fn from(item: String) -> Self {
        Text::from_string(item)
    }
}

impl From<Id> for Text {
    fn from(id: Id) -> Self {
        Text::from_string(id.to_string())
    }
}
