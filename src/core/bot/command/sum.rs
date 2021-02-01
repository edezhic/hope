use crate::core::*;

use crate::core::*;

pub fn sum(value: Value) -> Result<Option<Value>> {
    if let Value::List(list) = value {
        let mut sum = Number::zero();
        for val in list {
            if let Value::Number(num) = val {
                sum += num;
            } else {
                return Err(Error::Error("I can sum only numbers sry"))
            }
        }
        Ok(Some(Value::Number(sum)))
    } else {
        Err(Error::Error("I can sum only lists sry"))
    }
}