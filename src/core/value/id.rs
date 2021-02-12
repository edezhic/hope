use crate::core::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Scheme {
    Custom(Text),
    Display,
    Http,
    Ref,
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Id {
    scheme: Scheme,
    domain: Option<Text>,
    path: Option<Vec<Text>>,
}
impl Id {
    pub fn from_str(s: &str) -> Result<Id> {
        let scheme = Scheme::Custom(Text::from_str("temp"));
        let domain = Some(Text::from_str(s));
        let path = None;
        Ok(Id {
            scheme,
            domain,
            path,
        })
    }
    pub fn from_text(t: Text) -> Result<Id> {
        Ok(Id::from_str(t.as_str())?)
    }
}
