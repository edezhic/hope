#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_must_use,
    non_snake_case,
    unused_assignments
)]

use console_error_panic_hook;
pub use core::fmt;
pub use derive_more;
pub use lazy_static::lazy_static;
pub use petgraph::{stable_graph::{NodeIndex as Node, StableDiGraph as Graph}, dot::Dot};
pub use serde::{Deserialize, Serialize};
pub use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;

mod builder;
mod error;
mod parser;
mod token;
mod value;
pub use builder::build;
pub use error::{Error::*, Result};
pub use matches_macro::Matches;
pub use parser::Parser;
pub use token::{
    Algebra::*, Control::*, Function::*, Preposition::*, Relation::*, Selector::*, Token::*, *,
};
pub use value::{Value::*, *};

#[wasm_bindgen]
pub fn get_build(title: &str, body: &str) -> JsValue {
    console_error_panic_hook::set_once();
    let mut tokens = Parser::convert(title).unwrap();
    tokens.push((42, Linebreak));
    tokens.extend(Parser::convert(body).unwrap());
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
    "Z is 1, xyz is [x, y, z], abc is @abcd. Sum xyz and show.",
);
