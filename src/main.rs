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
extern crate derive_more;

mod error;
mod pieces;
mod script;
mod token;
mod value;
mod vocabulary;
pub use error::{Error, Result};
pub use pieces::Pieces;
pub use script::*;
pub use token::*;
pub use value::*;
pub use vocabulary::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");

    debug(
        "X is 0.5, y is 1.5. Add x to y, show result.",
    )?;

    /*
    script.debug("\
    Array1 is [1, 2], Array2 is [[1, 2], [3, 4]]. Array is (Array1 + Array2). \
    S is 0. For each x in the array add x to S.
    Label is { type: 'Label', text: S }, show it. \
    ")?;
    // T = [ V V ] T = [ [ V V ] [ V V ] ] . T = ( T O T ) . T = V . F T t T Cmd T t T . T = { T = V T = T } Cmd _ .

    script.debug("\
    CustomCommand X by Y.
    Filter X by Y, add 1, multiply by 2, return the result")?;
    // T T b T . Cmd T b T Cmd V Cmd b V F _

    script.debug("\
    For each message at @http://domain/path
    request @database with query from the content of the message,
    If the sum of result is less than 100, then response is 'Not enough',
    else response is 'Enough'.
    Send response to the sender of the message.\
    ")?;
    // Cmd T t V Cmd V b T s T s T . F Cmd s _ = T T V F T = V F T = V . Cmd T t T s T .
     */
    Ok(())
}
