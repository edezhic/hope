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
    #[matches(regex = r"^\.$")]
    Dot,
    #[matches(regex = r"^(\n|\p{Zl})$")]
    Linebreak,
    #[matches(regex = r"^(\]|\})$")]
    CollectionEnd,

    And,
    Or,
    Not,

    Do,
    Else,
    If,
    While,
    Return,
    Match,
    Then,
    Try,
    Panic,

    Where,
    Any,
    Each,

    Than,

    A(Algebra),
    C(Comparative),
    F(Function),
    P(Preposition),
    V(Value),
    #[matches(nothing)]
    Term(Text),
    //#[matches(nothing)]
    //Script(Id), // for custom scripts
    // + static?
    // + T(Type)
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
