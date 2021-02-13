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
    @ # & are literal markers for Id, Version and Seal (need smth for time)
    */
    //bot._do(r#"x as [@display, 1,5], y as {z: x, abc}. Show x and y"#)?;

    bot._do(r#"x as @http, y as 1.5. Show z of x and y"#)?;


    Ok(())
}
