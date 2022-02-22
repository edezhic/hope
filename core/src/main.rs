use hope_core::*;

/*
pub struct Script { 
    // -> enum + struct Auditor/Bot (which contains all debug stuff)?
    // only Text? only parsed?
    title: Text,
    body: Text,
    source: (Text, Text),
    name: Text,
    input: Option<Text>,
    args: Vec<(Modifier, Text)>,
    tokens: Vec<(usize, Text)>,
    compiled: ()
}

impl Script {
    // fn build { if Source { convert }; build algorithm }
}
*/

fn main() -> Result<()> {
    let mut tokens = Parser::convert(TESTS[0].0)?;
    tokens.push((42, Break));
    tokens.extend(Parser::convert(TESTS[0].1)?);
    println!("{:?}", tokens);
    Ok(())
}