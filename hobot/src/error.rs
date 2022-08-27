use crate::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error { // TODO https://crates.io/crates/thiserror ?
    Execution(String),
    Parsing(String),
    Regex(String),
    Message(&'static str),
    UnexpectedToken(Token, usize),
    ExpectedToken(Token),
}

impl std::convert::From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        if let regex::Error::Syntax(err) = e {
            Error::Regex(err)
        } else {
            Error::Message("something went wrong with regex")
        }
    }
}

impl std::convert::From<rust_decimal::Error> for Error {
    fn from(e: rust_decimal::Error) -> Self {
        Error::Message("something went wrong with numbers")
    }
}

impl std::convert::From<chrono::format::ParseError> for Error {
    fn from(e: chrono::format::ParseError) -> Self {
        Error::Message("something went wrong with chrono")
    }
}