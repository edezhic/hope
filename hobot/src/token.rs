use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches, OfType)]
pub enum Token {
    #[dont_match]
    Edge,
    And,
    #[dont_match]
    Input,
    Or,
    #[regex = r"^(?i)(result|this|it|that)$"]
    This,
    #[regex = r"^(?i)(:|=|is|are|equal)$"]
    Be,
    #[regex = r"^\.$"]
    Dot,
    #[regex = r"^(\n|\p{Zl})$"]
    Linebreak,
    #[regex = r"^(\]|\})$"]
    CollectionEnd,
    A(Algebra),
    C(Control),
    S(Selector),
    F(Function),
    V(Value),
    P(Preposition),
    R(Relation),
    #[dont_match]
    Term(Text),
    #[dont_match]
    Script(Id), // for custom scripts
                // + static?
                // + T(Type)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Control {
    Do,
    Else,
    If,
    While,
    Return,
    Match,
    Then,
    Try,
    Panic,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Selector {
    Where,
    Any,
    Each,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Relation {
    Than,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches, FunctionSyntax)]
pub enum Function {
    #[syntax(returns, args = "To")]
    Add,
    #[syntax(args = "From")]
    Substract,
    #[syntax(args = "By")]
    Filter,
    #[syntax(returns)]
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
    #[regex = r"^\($"]
    Start,
    #[regex = r"^\)$"]
    End,
    #[regex = r"^\+$"]
    Plus,
    #[regex = r"^\-$"]
    Minus,
    #[regex = r"^\*$"]
    Multiplication,
    #[regex = r"^/$"]
    Division,
    Mean,
    Deviation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
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
