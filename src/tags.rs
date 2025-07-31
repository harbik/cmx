use std::{fmt::Display, str::FromStr};

use crate::Error;

pub mod cmm;

mod profile_tags;
pub use profile_tags::ProfileTag;

pub mod colorspace;
pub mod technology;
pub mod typesignatures;


pub struct Tag(pub u32);

/// Represents an ICC profile signature, which is a 4-byte value that can be interpreted as an ASCII string.
impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_u32_as_string(f, self.0)
    }
}
fn format_u32_as_string(f: &mut std::fmt::Formatter<'_>, value: u32) -> std::fmt::Result {
    let bytes = value.to_be_bytes();
    let s = String::from_utf8_lossy(&bytes);
    if s.is_ascii() && s.len() == 4 {
        write!(f, "{}", s)
    } else {
        write!(f, "{:08X}", value)
    }
}

/// Parses a 4-character string into a `Tag`.
/// If the string is not exactly 4 characters long, it returns an error.
impl FromStr for Tag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Error::ParseError(format!("Signature must be exactly 4 characters long, got: {}", s)));
        }
        let bytes = s.as_bytes();
        let value = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok(Tag(value))
    }
}

impl From<Tag> for u32 {
    fn from(sig: Tag) -> Self {
        sig.0
    }
}

impl From<u32> for Tag {
    fn from(value: u32) -> Self {
        Tag(value)
    }
}