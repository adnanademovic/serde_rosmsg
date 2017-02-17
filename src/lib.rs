#![recursion_limit = "1024"]

extern crate byteorder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[doc(inline)]
pub use self::ser::Serializer;
#[doc(inline)]
pub use self::de::*;
#[doc(inline)]
pub use self::error::Error;

pub mod ser;
pub mod de;
pub mod error;
