use crate::core::*;

pub struct Spec<'a> {
    refs: &'a str, // T (#v) (&h): T(Op1), T(Op2), ...
    main: &'a str,
    ops: Vec<&'a str>, // T T(???) (m T). ... (return)
    // options: Option<&'a str>?
}

pub struct Process {
    // recursevily translate Specs
    // construct namespace(s?): link terms to specs, values and ops, ops' arguments
    // build graph(algorithm)
    algorithm: Graph,
    inputs: &Vec<Something>,
}