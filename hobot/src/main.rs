use hobot::*;

fn main() -> Result<()> {
    println!("{:?}\n{:?}", TEST.0, TEST.1);

    let mut tokens = Parser::convert(TEST.0)?;
    tokens.push((42, Linebreak));
    tokens.extend(Parser::convert(TEST.1)?);
    
    println!("{:?}", tokens);

    println!("{:?}", Dot::with_config(&build(tokens)?.graph, &[]));
    
    Ok(())
}