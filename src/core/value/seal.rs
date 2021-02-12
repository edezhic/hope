use crate::core::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Seal(Text);
// TODO Combine signatures, public keys and other crypto primitives as Seal? 
// Incapsulate private keys in some security module?
// scheme:value like SHA256(EDCSA_pub):AasjVasda147asc... ?
// struct { scheme: enum, value: binary(base57)? Text? }

impl Seal {
    pub fn from_text(t: Text) -> Result<Seal> {
        Ok(Seal(t))
    }
}

