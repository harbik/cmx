
use crate::tags::RawType;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

#[derive(Serialize)]
pub struct RawTypeToml{
    type_signature: String,
    data: Vec<u8>,
    hex: String,
}

#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
pub struct RawTagTypeLayout {
    /// Tag signature, must be `b"raw "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    /// Raw data bytes.
    data: [u8],
}

impl From<&RawType> for RawTypeToml {
    fn from(raw: &RawType) -> Self {
        let layout = 
            RawTagTypeLayout::ref_from_bytes(&raw.0).unwrap();

        Self {
            type_signature: String::from_utf8_lossy(&layout.signature).to_string(),
            data: layout.data.to_vec(),
            hex: hex::encode(&layout.data),
        }
    }
}