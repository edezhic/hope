#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_must_use,
    non_snake_case,
    unused_assignments
)]
#[macro_use]
extern crate lazy_static;
extern crate console_error_panic_hook;
extern crate derive_more;
use petgraph::graph::DiGraph;
use wasm_bindgen::prelude::*;

mod error;
mod parser;
mod linker;
mod token;
mod value;
pub use error::{Error::*, Result};
pub use parser::Parser;
pub use token::{Token::*, *};
pub use value::*;
pub use linker::link;

#[wasm_bindgen]
pub fn build(title: &str, body: &str) -> JsValue {
    console_error_panic_hook::set_once();
    let mut tokens = Parser::convert(title).unwrap();
    tokens.push((42, Newline));
    tokens.extend(Parser::convert(body).unwrap());
    let program = link(tokens.clone()).unwrap();
    JsValue::from_serde(&(tokens, program.graph)).unwrap()
}

#[wasm_bindgen]
pub fn get_test() -> JsValue {
    console_error_panic_hook::set_once();
    JsValue::from_serde(&TEST).unwrap()
}

pub const TEST: (&'static str, &'static str) = 
    ("Testscript X of Y", "Z is 1, xyz is [x, y, z]. Sum xyz and show.");