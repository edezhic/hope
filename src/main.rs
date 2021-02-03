#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

extern crate derive_more;

mod core;
use crate::core::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");
    let mut bot = Bot::init()?;
    /*
    Commands are executed in intuitive manner
    Terms are initialized with value Fact::truth()
    @ # * & are literal markers for Id, Version, Time and Seal
    Lists are defined in [] like [x, y, z]
    Structures are defined in {} like {x: x, y: y, z: z}, or simply {x, y, z} as a shorthand
    Expressions inside () evaluated into some Value with mathematical op precedence
    Comments inside ``
    */
    bot.perform(r#"Column as [1.333, 2, 3,5], table as {column, column2: [0, 0]}, struct as {table, flag}."#)?;
    bot.perform(r#"Show flag of struct. Set flag of struct to no. Show flag of struct."#)?;
    bot.perform(r#"Sum column of table of struct. Show result."#)?;

    Ok(())
}
