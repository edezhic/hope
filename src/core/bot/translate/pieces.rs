use crate::core::*;
use std::iter::Peekable;
use unicode_segmentation::UWordBounds;
pub struct Pieces<'a, 'b> {
    iter: Peekable<UWordBounds<'a>>,
    vocab: &'b Vocabulary,
    pub peek: Option<&'a str> 
}
impl<'a, 'b> Pieces<'a, 'b> {
    pub fn split(text: &'a Text, vocab: &'b Vocabulary) -> Pieces<'a, 'b> {
        let mut iter = text.split_by_word_bounds().peekable();
        let mut peek = None;
        if let Some(piece) = iter.peek() {
            peek = Some(*piece);
        }
        Pieces {
            iter,
            vocab,
            peek
        }
    }
    fn update_peek(&mut self) {
        if let Some(piece) = self.iter.peek() {
            self.peek = Some(*piece)
        } else {
            self.peek = None
        }
    }
    pub fn next(&mut self) -> Option<&'a str> {
        self.iter.next();
        self.skip();
        self.update_peek();
        self.peek
    }
    pub fn collect_until(&mut self, pattern: &regex::Regex, skip: bool) -> Text {
        self.iter.next();
        let mut text = Text::empty();
        while let Some(piece) = self.iter.next() {
            if pattern.is_match(piece) {
                break;
            } else {
                text.add(piece);
            }
        }
        if skip {
            self.iter.next();
        }
        self.skip();
        self.update_peek();
        text
    }
    pub fn collect_literal(&mut self) -> Text {
        self.collect_until(&self.vocab.skip, false)
    }
    pub fn skip(&mut self) {
        while let Some(piece) = self.iter.peek() {
            if self.vocab.skip.is_match(piece) {
                self.iter.next();
            } else {
                break;
            }
        }
    }
}