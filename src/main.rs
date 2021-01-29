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
        r#"Show "Hello world!", SomeVariable as { 1 + 4 / 2 } (3), Struct as { column1: { 1, 2, 3 }  }.
         Show sum column1 of Struct. Send this to @someone.
         Show #0.0.1-alpha, @scheme://authority/path, &signature, *29-01-2021_16:20.
         SomeFlag1, SomeFlag2. If SomeFlag1 and SomeFlag2, then sum 1, 2, 3 and show result. 
         Show "Done!""#,
    )?;
    
    /* 
    bot.perform(
        r#"Show "Hello World!""#,
    )?;
    */
    Ok(())
}
