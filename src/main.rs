#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_must_use, non_snake_case, unused_assignments)]
#![feature(let_chains)]
extern crate derive_more;

mod core;
use crate::core::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");
    let mut bot = Bot::init()?;
    
    //bot._do(r#"x as {a: 1.5, b: y of z, c}, y as 1.5. Show z of x and y"#)?;
    bot._do(r#"x as [1.5, @abc, "123"], y as 1.5. Show z of x and y"#)?;

    Ok(())
}
