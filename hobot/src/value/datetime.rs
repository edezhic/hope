use crate::*;
use chrono::{DateTime, Duration, TimeZone, Utc};
use std::str::FromStr;
use core::fmt;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Datetime {
    Stamp(DateTime<Utc>),
    //Duration(Duration), // TODO Add duration, but there is no serde support for this one
}

impl Datetime {
    pub fn from_text(t: Text) -> Result<Datetime> {
        let datetime = Utc.datetime_from_str(t.as_str(), "%d-%m-%Y_%H:%M")?;
        Ok(Datetime::Stamp(datetime))
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Datetime::Stamp(stamp) => write!(f, "T{}", stamp)
        }
    }
}