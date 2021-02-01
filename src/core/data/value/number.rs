use crate::core::Result;
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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
}
