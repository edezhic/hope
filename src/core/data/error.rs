use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Error {
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
            Error::Error("something gone wrong with regex")
        }
    }
}

impl From<iref::Error> for Error {
    fn from(e: iref::Error) -> Self {
        Error::Error("something gone wrong with iref")
    }
}

impl From<bigdecimal::ParseBigDecimalError> for Error {
    fn from(e: bigdecimal::ParseBigDecimalError) -> Self {
        Error::Error("something gone wrong with BigDecimal")
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(e: chrono::format::ParseError) -> Self {
        Error::Error("something gone wrong with chrono")
    }
}

impl From<getrandom::Error> for Error {
    fn from(e: getrandom::Error) -> Self {
        Error::RNGError
    }
}