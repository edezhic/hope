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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Value {
    Fact(Fact),
    Id(Id),
    Number(Number),
    List(List), 
    Seal(Seal),
    Structure(Structure),
    Text(Text),
    Time(Time),
    Version(Version),
}

impl Default for Value {
    fn default() -> Value { 
        Value::Fact(Fact::truth())
    }
}