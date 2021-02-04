use crate::core::Result;
use derive_more::{Add, AddAssign};
use rust_decimal::prelude::*;
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Add, AddAssign)]
pub struct Number {
    value: Decimal,
}

impl Number {
    pub fn from_str(value: &str) -> Result<Number> {
        let value = Decimal::from_str(value)?;
        Ok(Number { value })
    }
    pub fn from_string(value: String) -> Result<Number> {
        let value = Decimal::from_str(value.as_str())?;
        Ok(Number { value })
    }
    pub fn value(&self) -> &Decimal {
        &self.value
    }
    pub fn zero() -> Number {
        Number {
            value: Decimal::zero(),
        }
    }
}
