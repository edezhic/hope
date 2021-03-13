#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_must_use, non_snake_case, unused_assignments)]
extern crate derive_more;

mod core;
use crate::core::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");
    let mut bot = Bot::init()?;
    
    bot._do(r#"x as {a: 1.5, b: y of z, c}, y as [1.5, @abc, "123"]. Sum a of x and 2.0 and show result"#)?;
    
    Ok(())
}
