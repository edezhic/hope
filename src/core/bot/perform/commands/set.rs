use crate::core::*;

impl Command {
    pub fn set(target: &Value, value: Value) -> Result<Value> {
        unsafe {
            // yeah yeah that's bad
            let mutable_target = &mut *((target as *const Value) as *mut Value);
            *mutable_target = value;
        }
        Ok(Value::None)
    }
}
