#![recursion_limit = "1024"]

extern crate byteorder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub use self::ser::Serializer;
pub use self::de::*;
pub use self::error::Error;

mod ser;
mod de;
pub mod error;
