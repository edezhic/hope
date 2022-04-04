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
use wasm_bindgen::prelude::*;

mod error;
mod parser;
mod token;
mod value;
pub use error::{Error, Result};
pub use parser::Parser;
pub use token::{Token::*, *};
pub use value::*;

#[wasm_bindgen]
pub fn build(title: &str, body: &str) -> JsValue {
    console_error_panic_hook::set_once();
    let mut tokens = Parser::convert(title).unwrap();
    tokens.push((42, Break));
    tokens.extend(Parser::convert(body).unwrap());
    JsValue::from_serde(&tokens).unwrap()
}

#[wasm_bindgen]
pub fn get_tests() -> JsValue {
    console_error_panic_hook::set_once();
    JsValue::from_serde(&TESTS).unwrap()
}

pub const TESTS: [(&'static str, &'static str); 9] = [
    ("Testscript X of Y", 
    "Z is 1, xyz is [x, y, z], s is (x + y + z). Show s, sum and show xyz. If x is less than 2 and y is more than 3 then show {x, y, z, xyz, s}. 
    For each number in xyz log (number * 2). For each message at @/endpoint/path/ show content of the message. For each message from @URI return x.
    Request 'query' from @http://wikipedia.com and store result at @storage:wiki. Get 'query' from @storage:wiki and send to @scheme://domain/path.
    Select 'query' from @storage:x where element is more than 0 and (element * 2) is less than 10
    Match X: 0 - show 0, 1 - show 1 
    Sign Q in W as E for R. Wallet, my
    Raise 'error'? 'exception'? Hanlding and propagation
    GUI: layout, elements, styles, interactions(for each?). Plot/draw/model?
    System protocols: event, debug, env, networking, filesystem, web3? 
    Multiline constructs: flow, structure literals, others
    'IDE -> HOPE', 'core -> HOBot(runtime/VM)', 'Term -> Reference', 'Node' = 'Token'
    "),
    ("Termscript", "X is 1, Y is 2"),
    ("Listscript", "X is 1, list is [1.333, 2, 3,5, x]"),
    ("Structscript", "X is 1, structure is {x, flag: yes}"),
    ("Commandscript", "X is 1, substract 1 from x and show"),
    ("Ifscript", "X is 1, if x is less than 2 then show 'Ok' "),
    ("Formulascript", "X is 2, (2 + 2 * (x + 2))"),
    (
        "Argscript X",
        "If any X, then show 'running with an argument'",
    ),
    (
        "Chainscript",
        "Script1 X1 of command1 of X2 of X3 with Script2 of X4",
    ),
];
