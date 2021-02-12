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
pub use id::Id;
pub use list::List;
pub use number::Number;
pub use seal::Seal;
pub use structure::*;
pub use text::Text;
pub use time::Time;
pub use version::Version;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Value {
    None,
    Fact(Fact),
    Id(Id),
    Number(Number),
    List(List), 
    Seal(Seal),
    Structure(Structure),
    Text(Text),
    Time(Time),
    Version(Version),
    // FIXME add Tensor?
}

impl Value {
    pub fn flag() -> Value {
        Value::Fact(Fact::truth())
    }

    pub fn unsafe_set(&self, value: Value) {
        // forgive me god and rust compiler for mutating the immutable
        unsafe {
            // FIXME get rid of this abomination
            let mut_self = &mut *((self as *const Self) as *mut Self);
            *mut_self = value;
        }
    }
}

impl Default for Value {
    fn default() -> Value { 
        Value::None
    }
}