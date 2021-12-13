use hope_core::*;

fn main() -> Result<()> {
    let script = r#"save it at @structures/bestOne? asd
    asdasdasd"#;
    let tokens = Pieces::translate(script).unwrap();
    println!("{:?}", tokens);
    Ok(())
}