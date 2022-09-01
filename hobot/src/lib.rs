#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_must_use,
    non_snake_case,
    unused_assignments
)]
#![feature(if_let_guard)]
pub use core::fmt;
pub use derive_more;
pub use itertools::Itertools;
use itertools::MultiPeek;
pub use lazy_static::lazy_static;
pub use petgraph::{
    dot::Dot,
    stable_graph::{NodeIndex, StableDiGraph},
};
pub use regex::Regex;
pub use serde::{Deserialize, Serialize};
pub use std::collections::{HashMap, VecDeque};
pub use std::{iter::Peekable, vec::IntoIter};
pub use unicode_normalization::UnicodeNormalization;
pub use unicode_segmentation::{UWordBoundIndices, UnicodeSegmentation};
use wasm_bindgen::prelude::*;

mod builder;
mod error;
mod parser;
mod token;
mod token_impls;
mod value;
pub use builder::build;
pub use derive_matches::Matches;
pub use derive_of_type::OfType;
pub use derive_syntax::FunctionSyntax;
pub use error::{Error, Error::*, Result};
pub use parser::parse;
pub use token::{Algebra::*, Comparative::*, Function::*, Preposition::*, Token::*, *};
pub use value::{Value::*, *};

pub type IndexedTokenIter = MultiPeek<IntoIter<IndexedToken>>;
pub type TokenGraph = StableDiGraph<Token, Token>;

#[wasm_bindgen]
pub fn get_build(script: &str) -> Result<JsValue> {
    let indexed_tokens = parse(script)?;
    let program = build(indexed_tokens.clone())?;
    Ok(JsValue::from_serde(&(indexed_tokens, program.graph)).unwrap())
}

#[wasm_bindgen]
pub fn get_test() -> Result<JsValue> {
    Ok(JsValue::from_serde(&TEST).unwrap())
}

pub const TEST: &'static str = r#"Testscript X of Y
Z is 1, xyz is [x, y, sum of [x, y, z]], abc is @abcd. S is the sum of xyz. Show it. Substract 1 from s.
If this is less than 10 then show "hell yeah" and add z to s, else show it. Show "it works!".
"#;
