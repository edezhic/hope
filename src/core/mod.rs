mod bot;
mod error;
mod value;
pub use bot::*;
pub use error::{Error, Result};
pub use value::*;


pub struct Message { // FIXME 
    address: Id,
    content: Structure,
}