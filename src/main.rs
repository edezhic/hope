#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]

mod core;
use crate::core::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");
    let mut bot = Bot::init()?;

    bot.perform( 
        // Regular commands are executed in intuitive manner
        // expressions inside {} evaluated into some Value with mathematical op precedence
        // comments in ()
        // @ # * & are literal markers for Id, Version, Time and Seal
        // Collections, List and Structure, are in []
        // Value::Structure is a json-like thing, but with more primitive types and decimal instead of float for numbers 
        r#"Show "Hello" and " world!".
         SomeVariable as { 1 + 4 / 2 } (3), X as 1, Y as 2, Z as 3.
         List as [x, y, z], Struct as [a: x, y, z; b: x, y; c: x].
         Sum A of Struct, show the result and send it to @someone.
         Show #0.0.1-alpha, @scheme://authority/path, &signature, *29-01-2021_16:20.
         SomeFlag1, SomeFlag2. 
         If SomeFlag1 and SomeFlag2, then sum 1, 2, 3, and show result. Else show "unreachable command".
         Show "Done!""#,
    )?;
    
    /* 
    bot.perform(
        r#"Show "Hello World!""#,
    )?;
    */
    Ok(())
}
