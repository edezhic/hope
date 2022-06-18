use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Error { // TODO https://crates.io/crates/thiserror ?
    Execution(String),
    Parsing(String),
    Regex(String),
    Message(&'static str),
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        if let regex::Error::Syntax(err) = e {
            Error::Regex(err)
        } else {
            Error::Message("something went wrong with regex")
        }
    }
}

impl From<rust_decimal::Error> for Error {
    fn from(e: rust_decimal::Error) -> Self {
        Error::Message("something went wrong with numbers")
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(e: chrono::format::ParseError) -> Self {
        Error::Message("something went wrong with chrono")
    }
}