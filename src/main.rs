#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

mod core;
use crate::core::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");
    let mut bot = Bot::init()?;
    /*
    Commands are executed in intuitive manner
    Terms are initialized with value Fact::truth()
    @ # * & are literal markers for Id, Version, Time and Seal
    Collections(List and Structure), are defined with [] like [x, y, z] or [x: xx, y: yy, z: zz].
    Structure is a json-like thing, but with more primitive types and decimal instead of float for numbers
    Expressions inside {} evaluated into some Value with mathematical op precedence
    Comments in ()
    */
    bot.perform(r#"Xyz as "Hello world!". Show xyz."#)?;

    Ok(())
}
