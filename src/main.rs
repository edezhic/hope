#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_must_use,
    non_snake_case,
    unused_assignments
)]
extern crate derive_more;

mod core;
use crate::core::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");
    let mut bot = Bot::init()?;

    bot.debug("Label is { type: 'Label', text: '0', id: (2 + 2 * 2) }, show it")?;
    // T = { T = V T = V T = ( V O V O V ) } Cmd _

    bot.debug("Expect message at @http://domain/path, \
    request @database with query from the content of the message.
    If the sum of result is less than 100, then response is 'Not enough', \
    else response is 'Enough'.
    Send response to the sender of the message.")?;
    // Cmd T t V Cmd V b T s T s T . F Cmd s _ = T T V F T = V F T = V . Cmd T t T s T .

    bot.debug("CustomCommand X by Y. Filter X by Y, add 1, multiply by 2, return the result")?;
    // T T b T . Cmd T b T Cmd V Cmd b V F _

    Ok(())
}
