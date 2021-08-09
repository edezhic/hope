use crate::*;
use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

pub struct Pieces<'a> {
    iter: Peekable<UWordBounds<'a>>,
    pub peek: Option<&'a str> 
}
impl<'a> Pieces<'a> {
    pub fn split(text: &'a Text) -> Pieces<'a> {
        let mut iter = text.split_by_word_bounds().peekable();
        let mut pieces = Pieces {
            iter,
            peek: None
        };
        pieces.skim();
        pieces.update_peek();
        pieces
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
        self.skim();
        self.update_peek();
        self.peek
    }
    pub fn collect_until(&mut self, pattern: &regex::Regex, skim: bool) -> Text {
        self.iter.next();
        let mut text = Text::empty();
        while let Some(piece) = self.iter.next() {
            if pattern.is_match(piece) {
                break;
            } else {
                text.add(piece);
            }
        }
        if skim {
            self.iter.next();
        }
        self.skim();
        self.update_peek();
        text
    }
    pub fn collect_literal(&mut self) -> Text {
        self.collect_until(&SKIP, false)
    }
    pub fn skim(&mut self) {
        while let Some(piece) = self.iter.peek() {
            if SKIP.is_match(piece) {
                self.iter.next();
            } else {
                break;
            }
        }
    }
}