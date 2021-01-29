use crate::core::*;

fn get_random_buf() -> Result<[u8; 32]> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}

pub fn random_u32() -> Result<u32> {
    let a = get_random_buf()?;
    Ok(((a[0] as u32) <<  0) +
    ((a[1] as u32) <<  8) +
    ((a[2] as u32) << 16) +
    ((a[3] as u32) << 24))
}