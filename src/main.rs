#![allow(dead_code, unused_variables, unused_imports, unused_mut, unused_must_use, non_snake_case, unused_assignments)]
extern crate derive_more;

mod core;
use crate::core::*;

fn main() -> Result<()> {
    println!("\x1B[2J\x1B[1;1H Compilation ✓ =======================================");
    let mut bot = Bot::init()?;
    
    bot._do(r#"Lbl as { type: "Label", text: "1" }, Btn as { type: "Button", text: "+1" }, show Lbl and Btn. Expect event at @screen://btn/click, then set @screen://lbl/text to "2"."#)?;
    bot._do(r#"Request X from @database, sum y of x and send to @user"#)?;
    bot._do(r#"Define X as sum of (Y + Z)"#)?;
    bot._do(r#"Send @screen://lbl/text to Y"#)?;
    //bot._do(r#""#)?;
    
    // @hopes://spec/section/subsection?selectors+attributes#term
    // @hopes:#term
    // @hopes:?a=Y#X
    // Invoke @hopes://X/Y with @hopes:#Z
    // Expect @hopes:#X at/from @(source)
    // Send @hopes:#X to @(target)
    Ok(())
}
 