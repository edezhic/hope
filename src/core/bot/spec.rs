use crate::core::*;

pub struct Spec<'a> {
    refs: &'a str, // T (#v) (&h): T(Op1), T(Op2), ...
    main: &'a str,
    ops: Vec<&'a str>, // T (T???) (m T). ... (return)
    // options: Option<&'a str>? For CLI-like purposes godlike shit
}

pub struct Process {
    // recursevily translate and include Specs (or include and then translate?)
    // (add indexes to pieces and show errors with them)
    // construct namespace
    // replace Ts with Vs(Ids) and Os
    // (think rly hard how to do routing in @hopes://...; structures, lists and other weird cases)
    // ...
    // build graph(algorithm)

    // algorithm: Algorithm { graph, ...? }
    // inputs: Vec<&Node>,?
}

// Case::And -> Token::Join?

#[derive(Debug, PartialEq)]
pub struct OpSyntax {
    input: Option<Modifier>,
    gate: Option<Modifier>, // gates? -> ???
    output: bool,
}

