//! # Serde ROSMSG
//!
//! ROSMSG is a binary format used for communication between
//! [ROS](http://www.ros.org/) nodes.
//!
//! Message types are defined in ROS's own
//! [specification](http://wiki.ros.org/msg#Message_Description_Specification).
//! The linked specification contains information about supported data
//! structures. Serialization is performed by just writing the underlying
//! binary data in a Little Endian byte order. Fields that contain multiple
//! items are prefixed with a 32-bit length, in number of items. That applies
//! to variable sized arrays (like vectors) and strings.
//! Serialized data is prefixed by a 32-bit number, representing the number of
//! bytes in the serialized data, excluding that prefix.
//!
//!
//! # Examples
//!
//! If we wanted to serialize "Rust is great!", we would need to prefix the
//! string with 14, which is the string's length. Then we would need to prefix
//! that with 18, the combined length of the length annotation and the string
//! itself.
//!
//! ```rust
//! # use serde_rosmsg::{to_vec, from_slice};
//! let rosmsg_data = to_vec(&String::from("Rust is great!")).unwrap();
//! assert_eq!(rosmsg_data, b"\x12\0\0\0\x0e\0\0\0Rust is great!");
//! let rust_data: String = from_slice(&rosmsg_data).unwrap();
//! assert_eq!(rust_data, "Rust is great!");
//! ```
//!
//! Serialization is performed using [`serde`](https://crates.io/crates/serde).
//! Thus, if you want to serialize your own structures, you can use
//! [`serde_derive`](https://crates.io/crates/serde_derive)
//!
//! ```rust
//! extern crate serde_rosmsg;
//! #[macro_use]
//! extern crate serde_derive;
//! use serde_rosmsg::{to_vec, from_slice};
//!
//! fn main() {
//! #[derive(Debug,Serialize,Deserialize,PartialEq)]
//! struct Position {
//!     x: i16,
//!     y: i16,
//!     z: i16,
//! }
//!
//! let data = Position {
//!     x: 1025,
//!     y: -1,
//!     z: 5,
//! };
//!
//! let rosmsg_data = to_vec(&data).unwrap();
//! assert_eq!(rosmsg_data, [6, 0, 0, 0, 1, 4, 255, 255, 5, 0]);
//! let rust_data: Position = from_slice(&rosmsg_data).unwrap();
//! assert_eq!(rust_data, data);
//! }
//! ```

#![recursion_limit = "1024"]

extern crate byteorder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[doc(inline)]
pub use self::ser::*;
#[doc(inline)]
pub use self::de::*;
#[doc(inline)]
pub use self::error::Error;

pub mod ser;
pub mod de;
pub mod error;
mod datatests;
