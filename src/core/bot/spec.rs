use crate::core::*;

pub struct Spec<'a> {
    refs: &'a str, // T (#v) (&h): T(Op1), T(Op2), ...
    main: &'a str,
    ops: Vec<&'a str>, // T (T???) (m T). ... (return)
    // options: Option<&'a str>?
}

pub struct Process {
    // recursevily translate and include Specs
    // construct namespace(s?): link terms to values and ops, ops' arguments. 
    // replace T with R or smth alike? With Vs(Ids) and Os?
    // build graph(algorithm)
    algorithm: Algorithm
    //inputs: Vec<&Node>,?
}

pub struct Algorithm {

}

#[derive(Debug, PartialEq)]
pub struct Arguments {
    reference: ArgumentSyntax, // reference -> ???
    selection: ArgumentSyntax,
    targeting: ArgumentSyntax,
    binding: ArgumentSyntax,
}
#[derive(Debug, PartialEq)]
pub enum ArgumentSyntax {
    None,
    Optional,
    Required,
}

