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
    CollectionEnd,

    And, // X and Y = Tuple(X,Y)/List[X,Y]?
    Or,
    Either,
    Not,

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

    // These into Token::M(Modifier)?
    Any, // Any X = ?
    Each, // Each X = X.try_IntoIter?
    All, // All X = collect from X / = Each?

    Than,

    A(Algebra),
    C(Comparative),
    F(Function),
    P(Preposition),
    V(Value),
    #[matches(nothing)]
    Term(Text),
    #[matches(regex = "'")]
    Possessive,
    //#[matches(nothing)]
    // + Script(Id?) or C(Script) for custom scripts
    // + static?
    // + T(Type) with built-in types
    // + Description(?) for custom types
    // ??? syntax as term type ???
    // + Define for script/description definitions?
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Comparative {
    // -> Relative? Relational?
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Matches, FunctionSyntax)]
pub enum Function {
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
    Mean,
    Deviation,
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
