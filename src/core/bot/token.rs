use crate::core::*;
use core::fmt;

#[derive(Debug)]
pub enum Token {
    Case(Case), // part => Mod, part => higher-level Instruction(Vec<token>) thingy?
    Mod(Modifier), // How ...
    Op(Op), // ... to do ...
    Ref(Value), // ... stuff
    // Val(Value) + Term(Text)?
}

// Section {Linear, Condition, Conditioned, ... ?}
// Section X + Include X?
// Method Y + Invoke Y? Protocol?
// Module Z?
// ... Hash of X of Y? ... X of Op of Y? Dont read more than 1 piece at a time?

#[derive(Debug)]
pub enum Case { // -> ? Instruction?
    // None/Block(Vec<Token>)? Type/Struct Block = Vec<Token>? Methods/Protocols/Sections?
    // If/When/While/?:(?)
    // Condition(Vec<Token>)
    // Conditioned?
    And, // Modifier::Both?
    Any, // Modifier?
    Do, // -> Block? ??? -> Then?
    Each, // Modifier?
    Else, // -> Block?
    Identical, // Modifier?
    If, // -> Block(Conditions)?
    Not, // Modifier::Negation?
    Or, // Modifier::Either?
    Stop, // Modifier? Op?
    Then, // Modifier?
    When, // Block(Conditions)?
    While, // Block(Conditions)?
    // Where? Modifier? ???
}

#[derive(Debug)]
pub enum Op {
    Add,
    Assign,
    Await,
    Divide, 
    Multiply,
    Send,
    Sign, // Request @person to sign payload?
    Substract,
    Sum,
    Verify,
}

#[derive(Debug)]
pub enum Modifier { // -> ?
    Binding,
    Break, // Split ";|." and newlines into different modifiers?
    Selection,
    Targeting,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Case(case) => write!(f, "{:?}", case),
            Token::Mod(modifier) => write!(f, "{:?}", modifier),
            Token::Op(op) => write!(f, "{:?}", op),
            Token::Ref(value) => write!(f, "{}", value),
        }
    }
}