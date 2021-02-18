use core::fmt;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Fact(bool);

impl Fact {
    pub fn truth() -> Fact {
        Fact(true)
    }
    pub fn falsehood() -> Fact {
        Fact(false)
    }
}

impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            true => write!(f, "Y"),
            false => write!(f, "N")
        }
    }
}

