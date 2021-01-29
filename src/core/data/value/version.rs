use crate::core::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
