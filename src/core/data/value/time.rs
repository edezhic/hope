use crate::core::*;
use chrono::{DateTime, Duration, TimeZone, Utc};
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Time {
    Stamp(DateTime<Utc>),
    //Duration(Duration), // TODO Add duration, but there is no serde support for this one
}

impl Time {
    pub fn from_text(t: Text) -> Result<Time> {
        let time = Utc.datetime_from_str(t.as_str(), "%d-%m-%Y_%H:%M")?;
        Ok(Time::Stamp(time))
    }
}