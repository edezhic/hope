use crate::core::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Seal(Text);
// TODO Combine signatures, public keys and other crypto primitives as Seal? Incapsulate private keys in security module?
// scheme(SHA256/EDCSA_pub/...):value?

impl Seal {
    pub fn from_text(t: Text) -> Result<Seal> {
        Ok(Seal(t))
    }
}

