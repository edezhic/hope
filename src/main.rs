#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_must_use, non_snake_case, unused_assignments)]
extern crate derive_more;

mod core;
use crate::core::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");
    let mut bot = Bot::init()?;
    
    bot.debug(r#" SpecName #1.2: Abc, Def "#)?;
    
    bot.debug(" Label is { type: 'Label', text: '0', id: 1+1 }, show it.
                Expect message at @http://domain/path, \
                Request @database with query from content of the message, \
                then write the sum of result to @screen://label/text \
                and show 2 + 2 * 2 and show (2 + 2 * 2)
    ")?;

    /*
    bot.debug(r#" OpName X with Y
                Sum (X + 1) with Y and return result 
    "#)?;
     */

    //bot._do(r#"Define X as sum of (Y + Z)"#)?;
    
    Ok(())
}
 