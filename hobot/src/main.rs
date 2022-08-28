use hobot::*;

fn main() -> Result<()> {
    println!("{:?}", TEST);

    let indexed_tokens = parse(TEST)?;

    println!("{:?}", indexed_tokens);

    println!("{:?}", Dot::with_config(&build(indexed_tokens)?.graph, &[]));

    Ok(())
}
