//! MISON (minimal and simple object notation): a compact text format that reads back only through the type it
//! was written from. See the book in the repository for the rules.
#![doc = include_str!("../README.md")]

use std::io;

mod impls;
mod read;
mod write;

pub use read::{Decode, Reader};
pub use write::{Encode, RecordWriter, SeqWriter, Writer};

/// Derives `Encode` for a struct with named fields, as a record, or for an enum, as a variant.
///
/// Keys and labels are the Rust names of the fields and variants. Other types do not compile:
///
/// ```compile_fail
/// #[derive(mison::Encode)]
/// struct Id(u32);
/// ```
///
/// ```compile_fail
/// #[derive(mison::Encode)]
/// struct Unit;
/// ```
///
/// ```compile_fail
/// #[derive(mison::Encode)]
/// struct Empty {}
/// ```
///
/// ```compile_fail
/// #[derive(mison::Encode)]
/// union Bits { a: u32, b: f32 }
/// ```
///
/// ```compile_fail
/// #[derive(mison::Encode)]
/// enum Never {}
/// ```
pub use mison_derive::Encode;

/// Derives `Decode` for the same types as `Encode`, which it needs.
pub use mison_derive::Decode;

/// Writes `value` as a MISON text.
pub fn to_string<T: Encode + ?Sized>(value: &T) -> String {
    let mut w = Writer::new();
    value.write(&mut w);
    w.finish()
}

/// Writes `value` as a MISON text to `writer`.
pub fn to_writer<W: io::Write, T: Encode + ?Sized>(mut writer: W, value: &T) -> io::Result<()> {
    writer.write_all(to_string(value).as_bytes())
}

/// Reads a `T` from a whole MISON text.
pub fn from_str<T: Decode>(text: &str) -> Result<T, Error> {
    let mut r = Reader::new(text);
    let value = T::read(&mut r)?;
    r.finish()?;
    Ok(value)
}

/// Reads a `T` from a whole MISON text in UTF-8.
pub fn from_slice<T: Decode>(bytes: &[u8]) -> Result<T, Error> {
    let text = std::str::from_utf8(bytes).map_err(|e| Error(format!("invalid UTF-8 at byte {}", e.valid_up_to())))?;
    from_str(text)
}

/// Reads a `T` from all of `reader`'s MISON text in UTF-8.
pub fn from_reader<R: io::Read, T: Decode>(mut reader: R) -> Result<T, Error> {
    // shortcut: reads all input before parsing; parse while reading if inputs outgrow memory.
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).map_err(|e| Error(e.to_string()))?;
    from_slice(&bytes)
}

/// Why a text could not be read.
#[derive(Debug, Clone, PartialEq)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}
