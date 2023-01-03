use hobot::*;

fn main() -> Result<()> {
    println!("{:?}", TEST);
    let indexed_tokens = parse(TEST)?;
    for token in &indexed_tokens {
        print!("{:?}", token.token)
    }
    //println!("{:?}", indexed_tokens);
    println!("{:?}", Dot::with_config(&build(indexed_tokens)?.graph, &[]));
    Ok(())
}
