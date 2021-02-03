use crate::core::*;

impl Command {
    pub fn show(value: &Value) -> Result<Value> {
        println!("showing {:?}", value);
        Ok(Value::None)
    }
}
