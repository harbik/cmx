use std::{fmt::{Display, Debug}, str::FromStr};

use crate::Error;

mod cmm;
pub use cmm::Cmm;

mod device_class;
pub use device_class::DeviceClass;

mod profile_tag;
pub use profile_tag::ProfileTag;

mod colorspace;
pub use colorspace::ColorSpace;

pub mod technology;
pub mod typesignatures;

mod pcs;
pub use pcs::Pcs;

mod platform;
pub use platform::Platform;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag(pub u32);

/// Represents an ICC profile signature, which is a 4-byte value that can be interpreted as an ASCII string.
impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_u32_as_string(f, self.0)
    }
}

/// Represents an ICC profile signature, which is a 4-byte value that can be interpreted as an ASCII string.
impl Debug for Tag {
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
/// If the string is valid, it converts it to a `u32` by interpreting the bytes in big-endian order.
/// Example:
/// ```rust
/// use cmx::tags::Tag;
/// // using parse
/// let tag: Tag = "abcd".parse().unwrap();
/// assert_eq!(tag.0, 0x61626364); // 'abcd' in big-endian  
///
/// // Smaller strings are padded with spaces:
/// let tag: Tag = "XYZ".parse().unwrap();
/// assert_eq!(tag.0, 0x58595A20); // 'XYZ ' in big-endian  
///
/// // using from_str directly:
/// use std::str::FromStr;
/// let tag = Tag::from_str("mntr").unwrap();
/// assert_eq!(tag.0, 0x6D6E7472); // 'mntr' in big-endian
/// ```    
impl FromStr for Tag {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 4 || s.len() < 1 {
            return Err(Error::ParseError(format!("Signature must be between 1 and 4 characters - got: {}", s)));
        }
        // Pad the string to 4 characters with spaces if necessary
        let padded = format!("{: <4}", s); // Pad with spaces to ensure it's 4 characters
        let bytes = padded.as_bytes();
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