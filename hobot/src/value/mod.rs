mod id;
mod list;
mod number;
mod seal;
mod structure;
mod text;
mod datetime;
mod version;
pub use id::*;
pub use list::List;
pub use number::Number;
pub use seal::Seal;
pub use structure::*;
pub use text::Text;
pub use datetime::Datetime;
pub use version::Version;
use core::fmt;
use std::collections::{HashMap, VecDeque};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Value {
    Blank,
    Yes,
    No,
    I(Id),
    Num(Number),
    List(List), 
    Seal(Seal),
    Struct(Structure),
    Text(Text),
    Dt(Datetime),
    Ver(Version), 
}

impl Value {
    pub fn is_ref(&self) -> bool {
        if let Value::I(id) = self {
            if id.scheme == Scheme::Ref {
                return true
            }
        }
        false
    }
    pub fn new_struct() -> Self {
        Self::Struct(Structure {
            content: HashMap::new()
        })
    }
    pub fn new_list() -> Self {
        Self::List(List {
            values: VecDeque::new()
        })
    }
    /*
    pub fn unsafe_set(&self, value: Value) {
        // forgive me god and rust compiler for mutating the immutable
        unsafe {
            // FIXME get rid of this abomination
            let mut_self = &mut *((self as *const Self) as *mut Self);
            *mut_self = value;
        }
    }
     */
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Yes => write!(f, "Yes"),
            Value::No => write!(f, "No"),
            Value::I(id) => write!(f, "{}", id),
            Value::Num(number) => write!(f, "{}", number),
            Value::List(list) => write!(f, "{}", list),
            Value::Seal(seal) => write!(f, "{}", seal),
            Value::Struct(structure) => write!(f, "{}", structure),
            Value::Text(text) => write!(f, "{}", text),
            Value::Dt(time) => write!(f, "{}", time),
            Value::Ver(version) => write!(f, "{}", version),
            Value::Blank => write!(f, "blank"),
        }
    }
}