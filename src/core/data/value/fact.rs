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