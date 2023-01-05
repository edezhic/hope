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
    D(Descriptor),
    F(Flow),
    P(Preposition), // -> ?
    R(Relational), 
    V(Value),
    
    And, // X and Y = Tuple(X,Y)/List[X,Y]?
    Or,
    Either,
    Neither,
    Not,

    #[matches(nothing)]
    Edge,
    #[matches(nothing)]
    Input,
    #[matches(regex = r"^(?i)(:|=|is|are|equal)$")]
    Be,
    #[matches(regex = r"^,$")]
    Comma,
    #[matches(regex = r"^\.$")]
    Dot,
    #[matches(regex = r"^(\n|\p{Zl})$")]
    Linebreak,
    #[matches(regex = r"^(\]|\})$")] 
    CollectionEnd, // -> ListEnd + StructEnd?    
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
pub enum Descriptor {
    #[matches(nothing)]
    Term(Text), 
    #[matches(regex = r"^(?i)(this|it)$")]
    This,
    #[matches(regex = "'")]
    Possessive,
    Any, // Any X = ?
    Each, // Each X = X.intoIter and yield a single X if it's a singlular value?
    All, // All X = collect from X / = Each?
    //#[matches(regex = r"^(?i)(which|that|whose)$")] ?
    That, // Filter X stream values which satisfy conditions?

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Relational {
    Less,
    More,
    Contains,
    Than,
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
