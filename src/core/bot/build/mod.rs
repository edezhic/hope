use std::{iter::Peekable, vec::IntoIter};
use crate::core::*;

// Spec :: Section :: Tokens
// Spec X, Include X
// Section Y with { arg1: value1, arg2: value2, ... } `default values for args`
// Invoke Y (with { ... } `optional args override`)

struct Spec {
    // Vec<Section>?
}
struct Section {
    //Vec<Block/Op>? Vec<Step{Op,...}>?
}

impl Bot {
    pub fn build(&mut self, tokens: Vec<Token>) -> Result<()> {
        let mut tokens = &mut tokens.into_iter().peekable();
        Ok(())
    }
}