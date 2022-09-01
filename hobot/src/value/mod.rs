mod datetime;
mod id;
mod list;
mod number;
mod seal;
mod structure;
mod text;
mod version;
use crate::*;
pub use datetime::Datetime;
pub use id::*;
pub use list::List;
pub use number::Number;
pub use seal::Seal;
pub use structure::*;
pub use text::Text;
pub use version::Version;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Matches)]
pub enum Value {
    Blank,
    #[matches(regex = r"^(?i)(true|yes|ok)$")]
    Yes,
    #[matches(regex = r"^(?i)(false|no)$")]
    No,
    #[matches(nothing)]
    I(Id),
    #[matches(nothing)]
    Num(Number),
    #[matches(nothing)]
    Lst(List),
    #[matches(nothing)]
    Sl(Seal),
    #[matches(nothing)]
    Struct(Structure),
    #[matches(nothing)]
    Txt(Text),
    #[matches(nothing)]
    Dt(Datetime),
    #[matches(nothing)]
    Ver(Version),
}

impl Value {
    pub fn new_struct() -> Self {
        Self::Struct(Structure {
            content: HashMap::new(),
        })
    }
    pub fn new_list() -> Self {
        Self::Lst(List {
            values: VecDeque::new(),
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
            Value::Lst(list) => write!(f, "{}", list),
            Value::Sl(seal) => write!(f, "{}", seal),
            Value::Struct(structure) => write!(f, "{}", structure),
            Value::Txt(text) => write!(f, "{}", text),
            Value::Dt(time) => write!(f, "{}", time),
            Value::Ver(version) => write!(f, "{}", version),
            Value::Blank => write!(f, "blank"),
            //Value::Model {} => write!(f, "Model"),
        }
    }
}
