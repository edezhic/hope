#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Fact(bool);

impl Fact {
    pub fn truth() -> Fact {
        Fact(true)
    }
    pub fn falsehood() -> Fact {
        Fact(false)
    }
}