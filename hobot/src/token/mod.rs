mod token_impl;
mod token_iter;
pub use token_iter::IndexedTokensIter;

use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexedToken {
    pub index: usize,
    pub token: Token,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches, OfType)]
pub enum Token {
    #[matches(nothing)]
    Edge,
    #[matches(nothing)]
    Input,
    #[matches(regex = r"^(?i)(result|this|it|that)$")]
    This,
    #[matches(regex = r"^(?i)(:|=|is|are|equal)$")]
    Be,
    #[matches(regex = r"^,$")]
    Comma,
    #[matches(regex = r"^\.$")]
    Dot,
    #[matches(regex = r"^(\n|\p{Zl})$")]
    Linebreak,
    #[matches(regex = r"^(\]|\})$")] 
    CollectionEnd, // -> ?

    // ? Control-like smth
    Do,
    Else,
    If,
    While,
    Match,
    Then,
    Return, 
    Yield,
    Try,
    Panic,

    //#[matches(regex = r"^(?i)(which|that|whose)$")] ?
    Where, // X where conds = Xiter.filter(x matches conds)? 
    Which, // Filter X where conditions?
    // => S(Selector)?

    // Conjunctions/Junctions? => Relational?
    And, // X and Y = Tuple(X,Y)/List[X,Y]?
    Or,
    Either,
    Neither,
    Not, // => Determiner?

    Than, // => Relational?

    // These into D(Determiner)? S(Selector)
    Any, // Any X = ?
    Each, // Each X = X.intoIter and yield a single X if it's a singlular value?
    All, // All X = collect from X / = Each?
    #[matches(regex = "'")]
    Possessive, // => Determiner?

    A(Algebra),
    C(Command), // -> C(Command)? + Command::Custom or smth alike
    D, // Determiner? 
    P(Preposition),
    R(Relational), 
    T, // T(Type)/D(Description)/D(Definition) for types etc
    V(Value),
    #[matches(nothing)]
    Term(Text),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Relational {
    Less,
    More,
    Contains,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Syntax {
    pub expects_input: bool,
    pub expected_args: Vec<Preposition>,
    pub returns: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Matches, CommandSyntax)]
pub enum Command {
    #[syntax(args = "To", returns)]
    Add,
    #[syntax(args = "From", returns)]
    Substract,
    #[syntax(args = "By")]
    Filter,
    #[syntax(no_input, args = "Of", returns)]
    Sum,
    #[syntax(args = "To")]
    Send,
    #[syntax(returns, args = "From")]
    Get,
    #[syntax(args = "By")]
    Sort,
    Show,
    #[syntax(args = "As")]
    Sign,
    #[syntax(args = "By")]
    Group,
    #[syntax(args = "From")]
    Select,
    // Join? Remove/Delete?
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Algebra {
    #[matches(regex = r"^\($")]
    Start,
    #[matches(regex = r"^\)$")]
    End,
    #[matches(regex = r"^\+$")]
    Plus,
    #[matches(regex = r"^\-$")]
    Minus,
    #[matches(regex = r"^\*$")]
    Multiplication,
    #[matches(regex = r"^/$")]
    Division,
    // matches ???
    Mean, // -> Average?
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Matches)]
pub enum Preposition {
    For,
    With,
    By,
    Of,
    From,
    To,
    In,
    At,
    As,
}
