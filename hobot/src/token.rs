use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexedToken {
    pub index: usize,
    pub token: Token,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Token {
    A(Algebra),
    C(Command),
    F(Flow),
    R(Relation), 
    S(Spec),
    V(Value),
    
    And, // X and Y = Tuple(X,Y)/List[X,Y]?
    Or,
    Either,
    Neither,
    Not,

    #[matches(nothing)]
    Term(Text), 
    #[matches(regex = "'")]
    Possessive,
    #[matches(regex = r"^(?i)(this|it)$")]
    This,
    #[matches(regex = r"^(?i)(that|which|where)$")]
    That,

    #[matches(nothing)]
    Edge,
    #[matches(nothing)]
    Input,
    #[matches(regex = r"^(?i)(:|=|is|are)$")]
    Be,
    #[matches(regex = r"^,$")]
    Comma,
    #[matches(regex = r"^\.$")]
    Dot,
    #[matches(regex = r"^(\n|\p{Zl})$")]
    Linebreak,
    #[matches(regex = r"^(\]|\})$")] 
    CollectionEnd, // -> ListEnd + StructEnd? Yes please   
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Flow {
    Do,
    Else,
    If,
    While,
    When,
    Then,
    Return, 
    Yield,
    Try,
    Panic,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Relation {
    Less,
    More,
    Contains,
    Than,
    Equal,
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
pub enum Spec {
    For,
    With,
    By,
    Of,
    From,
    To,
    In,
    At,
    As,
    Any,
    Each, 
    All,
}
