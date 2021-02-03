use crate::core::*;

impl Command {
    pub fn sum(value: &Value) -> Result<Value> {
        if let Value::List(list) = value {
            let mut sum = Number::zero();
            for val in list.into_iter() {
                if let Value::Number(num) = val {
                    sum += num.clone();
                } else {
                    return Err(Error::Error("I can sum only numbers sry"));
                }
            }
            Ok(Value::Number(sum))
        } else {
            Err(Error::Error("I can sum only lists sry"))
        }
    }
}
