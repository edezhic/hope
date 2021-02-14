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
        let scheme = Scheme::Custom(Text::from_str("dummy"));
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

    pub fn get_term(&self) -> Result<Text> {
        if let Some(segments) = self.path && Scheme::Ref = self.scheme {
            if segments.len() == 1 {
                Ok(segments[0].clone())
            } else {
                Err(Error::Error("Invalid term"))
            }
        } else {
            Err(Error::Error("Not a term"))
        }
    }

    pub fn reference(selectors: Vec<Text>) -> Id {
        Id {
            scheme: Scheme::Ref,
            domain: None,
            path: Some(selectors),
        }
    }

    pub fn ref_result() -> Id {
        Id {
            scheme: Scheme::Ref,
            domain: None,
            path: None,
        }
    }
}
