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
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::console::*;

mod algorithm;
mod error;
mod pieces;
mod tokens;
mod value;
pub use algorithm::*;
pub use error::{Error, Result};
pub use pieces::*;
pub use tokens::{*, Token::*};
pub use value::*;

#[wasm_bindgen]
pub fn tokenize(script: &str) -> JsValue {
    console_error_panic_hook::set_once();
    let tokens = Pieces::translate(script).unwrap();
    JsValue::from_serde(&tokens).unwrap()
}