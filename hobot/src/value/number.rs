use crate::*;
use derive_more::{Add, AddAssign, DivAssign};
use rust_decimal::prelude::*;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Add, AddAssign, DivAssign, PartialEq)]
pub struct Number {
    value: Decimal,
}

impl Number {
    pub fn from_str(str: &str) -> Result<Number> {
        let value = Decimal::from_str(str)?;
        Ok(Number { value })
    }
    pub fn from_string(string: String) -> Result<Number> {
        let value = Decimal::from_str(string.as_str())?;
        Ok(Number { value })
    }
    pub fn from_integer(integer: i64) -> Result<Number> {
        let value = Decimal::from_i64(integer).unwrap();
        Ok(Number { value })
    }
    pub fn from_percentage(&mut self) -> Result<()> {
        self.value /= Number::from_integer(100)?.value;
        Ok(())
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
