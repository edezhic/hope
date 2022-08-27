use crate::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Scheme { // + system/network/other protocols
    Custom(Text),
    Screen,
    Http,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scheme::Screen => write!(f, "screen"),
            Scheme::Http => write!(f, "http"),
            Scheme::Custom(scheme) => write!(f, "{}", scheme)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Id {
    pub scheme: Scheme,
    pub domain: Option<Text>,
    pub path: Option<Path>,
}
impl Id {
    pub fn from_str(s: &str) -> Result<Id> {
        let scheme = Scheme::Custom(Text::from_str(s));
        let domain = None;
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

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.domain {
            Some(domain) => write!(f, "@{}://{}/{:?}", self.scheme, domain, self.path),
            None => {
                if let Some(path) = &self.path {
                    write!(f, "@{}:{}", self.scheme, path)
                } else {
                    write!(f, "@{}", self.scheme)
                }
            }
        }
    }
}