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
    @ # * & are literal markers for Id, Version, Time and Seal
    Lists are defined in [] like [x, y, z]
    Structures are defined in {} like {x: x, y: y, z: z}, or simply {x, y, z} as a shorthand
    Expressions inside () evaluated into some Value with mathematical op precedence
    Comments inside ``
    */
    bot._do(r#"Column1 as [1.333, 2, 3,5], table as {column1, column2: [0, 0]}, struct as {table, flag}."#)?;
    bot._do(r#"Set flag of struct to no. Sum column1 of table of struct. Show result."#)?; 
    bot._do(r#"X as 2, Y as 3. Show (x + x * y) `expressions not working yet :(`"#)?;

    Ok(())
}
