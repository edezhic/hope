use crate::core::*;
use iref::*;

// schemes: hope, http[s], ipfs, ..., custom?
// authorities: person, domain, ..., custom?

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(into = "Text")]
#[serde(from = "Text")]
pub struct Id(IriRefBuf);
impl Id {
    pub fn from_str(s: &str) -> Result<Id> {
        let iri = IriRefBuf::new(s)?;
        Ok(Id(iri))
    }
    pub fn from_text(t: Text) -> Result<Id> {
        Ok(Id::from_str(t.as_str())?)
    }
    pub fn to_string(&self) -> String {
        self.0.as_str().to_owned()
    }
}

impl From<Text> for Id {
    fn from(text: Text) -> Self {
        Id(IriRefBuf::new(&text.as_str()).unwrap())
    }
}
