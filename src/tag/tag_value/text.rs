// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use serde::Serialize;
use zerocopy::{FromBytes, Immutable, KnownLayout, Unaligned};

use crate::tag::tag_value::TextType;

/// Represents the raw memory layout of an ICC `TextType` tag.
#[repr(C)]
#[derive(FromBytes, KnownLayout, Unaligned, Immutable)]
struct TextTypeLayout {
    /// TagValue signature, must be `b"XYZ "`.
    signature: [u8; 4],
    /// Reserved, must be 0.
    _reserved: [u8; 4],
    text: [u8],
}

// Serializable structs for each tag type
#[derive(Serialize)]
pub struct TextTypeToml {
    text: String,
}

/// Parses the raw data wrapped in XYZType into a XYZTypeToml instance,
/// as used
impl From<&TextType> for TextTypeToml {
    fn from(text: &TextType) -> Self {
        let layout = TextTypeLayout::ref_from_bytes(&text.0).unwrap();
        // Take content up to the first NUL (0x00) byte, per ICC 'text' (ASCII, NUL-terminated)
        let bytes = &layout.text;
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        let text = String::from_utf8_lossy(&bytes[..end]).into_owned();

        Self { text }
    }
}
