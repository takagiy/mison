//! MISON (minified and simple object notation): a compact text format for serde that reads back only
//! through the type it was written from. See `SPEC.md` for the rules.
#![doc = include_str!("../README.md")]

mod de;
mod ser;

pub use de::from_str;
pub use ser::to_string;

/// Why a value could not be written or a text could not be read.
#[derive(Debug, Clone, PartialEq)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(message: T) -> Self {
        Error(message.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(message: T) -> Self {
        Error(message.to_string())
    }
}
