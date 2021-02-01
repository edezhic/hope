use crate::core::Result;
use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;
use derive_more::{Add, AddAssign};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Add, AddAssign)]
pub struct Number(BigDecimal);

impl Number {
    pub fn from_str(value: &str) -> Result<Number> {
        let number = BigDecimal::from_str(value)?;
        Ok(Number(number))
    }
    pub fn from_string(value: String) -> Result<Number> {
        let number = BigDecimal::from_str(value.as_str())?;
        Ok(Number(number))
    }
    pub fn value(&self) -> &BigDecimal {
        &self.0
    }
    pub fn zero() -> Number {
        Number(BigDecimal::zero())
    }
}