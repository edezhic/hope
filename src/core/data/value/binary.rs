#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Binary(Vec<u8>); // For anything in non-human-readable format?
impl Binary {
    pub fn from_array(value: &[u8]) -> Binary {
        Binary(value.iter().cloned().collect())
    }
}
