mod fact;
mod id;
mod list;
mod number;
mod seal;
mod structure;
mod text;
mod time;
mod version;
pub use fact::Fact;
pub use id::*;
pub use list::List;
pub use number::Number;
pub use seal::Seal;
pub use structure::*;
pub use text::Text;
pub use time::Time;
pub use version::Version;
use core::fmt;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Value {
    Fact(Fact), // -> Bool? Binary(!)? Something else?
    Id(Id),
    Number(Number),
    List(List), 
    Seal(Seal),
    Structure(Structure),
    Text(Text),
    Time(Time),
    Version(Version),
    None,
}

impl Value {
    pub fn truth() -> Value {
        Value::Fact(Fact::truth())
    }
    pub fn falsehood() -> Value {
        Value::Fact(Fact::falsehood())
    }
    pub fn is_ref(&self) -> bool {
        if let Value::Id(id) = self {
            if id.scheme == Scheme::Ref {
                return true
            }
        }
        false
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
            Value::Fact(fact) => write!(f, "{}", fact),
            Value::Id(id) => write!(f, "{}", id),
            Value::Number(number) => write!(f, "{}", number),
            Value::List(list) => write!(f, "{}", list),
            Value::Seal(seal) => write!(f, "{}", seal),
            Value::Structure(structure) => write!(f, "{}", structure),
            Value::Text(text) => write!(f, "{}", text),
            Value::Time(time) => write!(f, "{}", time),
            Value::Version(version) => write!(f, "{}", version),
            Value::None => write!(f, "None"),
        }
    }
}