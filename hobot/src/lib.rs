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
mod value;
pub use builder::build;
pub use derive_matches::Matches;
pub use derive_of_type::OfType;
pub use derive_syntax::FunctionSyntax;
pub use error::{Error, Error::*, Result};
pub use parser::parse;
pub use token::{Algebra::*, Comparative::*, Function::*, Preposition::*, Token::*, *};
pub use value::{Value::*, *};

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


pub static TEST: &'static str = r#" Valuate product with promos in currency
AccumulatedDiscount is the sum of each promos' discount. If accumulatedDiscount is equal or more than 100% then return 0.
If the currency is "dollar" then conversionRate is 1, else get the exchangeRate from (@http://exchange.com/dollar/to/.currency) and store it in the converstionRate.
Return ((product's price * (1 - accumulatedDiscount)) * conversionRate)
"#;
