mod data;
mod bot;
pub use data::*;
pub use bot::*;

// TODO remove those?
pub type Nothing = ();
pub const NOTHING: Nothing = ();
#[inline(always)]
pub fn not(v: bool) -> bool {
    !v
}
