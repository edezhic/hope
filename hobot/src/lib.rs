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
use console_error_panic_hook;
pub use core::fmt;
pub use derive_more;
pub use lazy_static::lazy_static;
pub use petgraph::{
    dot::Dot,
    stable_graph::{NodeIndex, StableDiGraph},
};
pub use regex::Regex as R;
pub use serde::{Deserialize, Serialize};
pub use std::collections::{HashMap, VecDeque};
pub use std::{iter::Peekable, vec::IntoIter};
pub use unicode_segmentation::{UWordBoundIndices, UnicodeSegmentation};
pub use unicode_normalization::UnicodeNormalization;
use wasm_bindgen::prelude::*;

mod builder;
mod error;
mod parser;
mod token;
mod value;
pub use builder::build;
pub use error::{Error::*, Result};
pub use derive_matches::Matches;
pub use derive_of_type::OfType;
pub use derive_syntax::FunctionSyntax;
pub use parser::parse;
pub use token::{
    Algebra::*, Control::*, Function::*, Preposition::*, Relation::*, Selector::*, Token::*, *,
};
pub use value::{Value::*, *};

pub type IndexedToken = (usize, Token);
pub type IndexedTokenIter = Peekable<IntoIter<IndexedToken>>;
pub type TokenGraph = StableDiGraph<Token, Token>;

#[wasm_bindgen]
pub fn get_build(title: &str, body: &str) -> JsValue {
    console_error_panic_hook::set_once();
    let mut tokens = parse(title).unwrap();
    tokens.push((42, Linebreak));
    tokens.extend(parse(body).unwrap());
    let program = build(tokens.clone()).unwrap();
    JsValue::from_serde(&(tokens, program.graph)).unwrap()
}

#[wasm_bindgen]
pub fn get_test() -> JsValue {
    console_error_panic_hook::set_once();
    JsValue::from_serde(&TEST).unwrap()
}

pub const TEST: (&'static str, &'static str) = (
    "Testscript X of Y",
    r#"Z is 1, xyz is [x, y, z], abc is @abcd. Sum xyz and show. If xyz is less than 5 then show "hell yeah", else show "oh no"."#,
);
