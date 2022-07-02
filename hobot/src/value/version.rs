use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Version {
    major: u16,
    minor: Option<u16>,
    patch: Option<u32>,
    rc: Option<String>,
    meta: Option<String>,
}

impl Version {
    pub fn from_text(t: Text) -> Result<Version> {
        Ok(Version {
            major: 0,
            minor: None,
            patch: None,
            rc: None,
            meta: Some(String::from(t.as_str()))
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.major);
        if let Some(minor) = &self.minor {
            write!(f, ".{}", minor);
            if let Some(patch) = &self.patch {
                write!(f, ".{}", patch);
                if let Some(rc) = &self.rc {
                    write!(f, "-{}", rc);
                    if let Some(meta) = &self.meta {
                        write!(f, "-{}", meta);
                    }
                }
            }
        }
        write!(f, "")
    }
}

