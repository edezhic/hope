use core::fmt;
use crate::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Scheme {
    Custom(Text),
    Screen,
    Http,
    Ref,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scheme::Ref => write!(f, "ref"),
            Scheme::Screen => write!(f, "screen"),
            Scheme::Http => write!(f, "http"),
            Scheme::Custom(scheme) => write!(f, "{}", scheme)
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Path(Vec<Text>);

impl Path {
    pub fn new() -> Path {
        Path(Vec::new())
    }
    pub fn from_vec(vec: Vec<Text>) -> Path {
        Path(vec)
    }
    pub fn empty(&self) -> bool {
        self.0.len() == 0
    }
    pub fn single(&self) -> bool {
        self.0.len() == 1
    }
    pub fn first_selector(&self) -> Text {
        self.0[0].clone()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length = self.0.len();
        if length > 0 {
            for index in (0..).take(length - 1) {
                write!(f, "{}/", &self.0[index]);
            }
            if let Some(selector) = self.0.get(length - 1) {
                write!(f, "{}", selector);
            }
        }
        write!(f, "")
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Id {
    scheme: Scheme,
    domain: Option<Text>,
    path: Path,
}
impl Id {
    pub fn from_str(s: &str) -> Result<Id> {
        let scheme = Scheme::Custom(Text::from_str(s));
        let domain = None;
        let path = Path::new();
        Ok(Id {
            scheme,
            domain,
            path,
        })
    }
    pub fn from_text(t: Text) -> Result<Id> {
        Ok(Id::from_str(t.as_str())?)
    }

    pub fn get_name(&self) -> Result<Text> {
        if !self.path.empty() {
            // check Scheme::Hopes = self.scheme?
            if self.path.single() {
                Ok(self.path.first_selector())
            } else {
                Err(Error::Error("Invalid name"))
            }
        } else {
            Err(Error::Error("Not a name"))
        }
    }

    pub fn reference(s: &str) -> Id {
        Id {
            scheme: Scheme::Ref,
            domain: Some(Text::from_str(s)),
            path: Path::new(),
        }
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.domain {
            Some(domain) => write!(f, "@{}://{}/{}", self.scheme, domain, self.path),
            None => {
                if self.path.empty() {
                    write!(f, "@{}", self.scheme)
                } else {
                    write!(f, "@{}:{}", self.scheme, self.path)
                }
            }
        }
    }
}