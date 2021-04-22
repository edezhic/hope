#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_must_use, non_snake_case, unused_assignments)]
extern crate derive_more;

mod core;
use crate::core::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");
    let mut bot = Bot::init()?;
    
    bot._do(r#" SpecName #1.2 &hash: Abc, Def
    "#)?;
    
    bot._do(r#" Set label to be { type: "Label", text: "0" }, show it
                Expect request at @http://domain/path, send query of request to @database and expect result,
                then set @screen://label/text to sum of result
    "#)?;
    // Send request to @abc (and expect response)? Op::Request -> Trash?

    bot._do(r#" OpName X of Y
                Do this and that with X and return result 
    "#)?;

    //bot._do(r#"Define X as sum of (Y + Z)"#)?;
    
    Ok(())
}
 