use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Error { // TODO https://crates.io/crates/thiserror ?
    ExecutionError(String),
    ParsingError(String),
    RegexError(String),
    RNGError,
    Error(&'static str),
    NoMatch,
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        if let regex::Error::Syntax(err) = e {
            Error::RegexError(err)
        } else {
            Error::Error("something went wrong with regex")
        }
    }
}

impl From<iref::Error> for Error {
    fn from(e: iref::Error) -> Self {
        Error::Error("something went wrong with iref")
    }
}

impl From<rust_decimal::Error> for Error {
    fn from(e: rust_decimal::Error) -> Self {
        Error::Error("something went wrong with numbers")
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(e: chrono::format::ParseError) -> Self {
        Error::Error("something went wrong with chrono")
    }
}