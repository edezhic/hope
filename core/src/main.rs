use hope_core::*;

fn main() -> Result<()> {
    let script = r#"If any X, then show "with an argument"
    asdasdasd"#;
    let tokens = Pieces::translate(script).unwrap();
    println!("{:?}", tokens);
    Ok(())
}