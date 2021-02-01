use crate::core::*;

pub fn show(value: Value) -> Result<Option<Value>> {
    println!("showing {:?}", value);
    Ok(None)
}