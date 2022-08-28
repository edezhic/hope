use crate::*;
use derive_more::{Add, AddAssign};
use rust_decimal::prelude::*;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Add, AddAssign, PartialEq)]
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

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
