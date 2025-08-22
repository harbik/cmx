// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::tag::tagdata::TextData;

/// Represents the raw memory layout of an ICC `TextData` tag.
#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
struct Layout {
    /// TagData signature, must be `b"XYZ "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    text: [u8],
}

#[repr(C)]
#[derive(FromBytes, IntoBytes, KnownLayout, Unaligned, Immutable)]
struct WriteLayout {
    /// TagData signature, must be `b"XYZ "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
}

impl WriteLayout {
    pub fn new() -> Self {
        Self {
            signature: super::DataSignature::TextData.into(),
            _reserved: [0; 4],
        }
    }
}

// Serializable structs for each tag type
#[derive(Serialize)]
pub struct TextType {
    text: String,
}

/// Parses the raw data wrapped in XYZData into a XYZDataToml instance,
/// as used
impl From<&TextData> for TextType {
    fn from(text: &TextData) -> Self {
        let layout = Layout::ref_from_bytes(&text.0).unwrap();
        // Take content up to the first NUL (0x00) byte, per ICC 'text' (ASCII, NUL-terminated)
        let bytes = &layout.text;
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        let text = String::from_utf8_lossy(&bytes[..end]).into_owned();

        Self { text }
    }
}

impl TextData {
    pub fn set_text(&mut self, text: &str) {
        // Enusure the text is valid Ascii and fits in the tag
        if !text.is_ascii() {
            panic!("TextData only supports ASCII text");
        } 

        let mut buf = Vec::with_capacity(std::mem::size_of::<WriteLayout>() + text.len() + 1);
        buf.extend_from_slice(WriteLayout::new().as_bytes());

        buf.extend_from_slice(text.trim_end_matches('\0').as_bytes());
         
        buf.push(0); // Append NUL terminator
        self.0 = buf;
    }
}
