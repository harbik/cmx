// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

//! This module provides a fully parsed representation of an ICC `textDescriptionData` tag.
//! It includes methods to parse the tag from bytes and retrieve the ASCII and Unicode descriptions.
//!
//! # Notes
//! - The use of this type is deprecated in favor of the `multiLocalizedUnicodeData`.
//! - Only

use serde::Serialize;

use std::ffi::CString;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::str;

/// A fully parsed representation of an ICC `textDescriptionData` tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TextDescriptionType {
    /// The 7-bit ASCII description.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ascii: String,
    /// The Unicode description.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub unicode: String,
    /// The Unicode language code for the Unicode description.
    #[serde(skip_serializing_if = "crate::is_zero")]
    pub unicode_language_code: u32,
    /// The ScriptCode code for the Macintosh script name.
    #[serde(skip_serializing_if = "crate::is_zero")]
    pub scriptcode_code: u16,
    /// The Macintosh script name.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub mac_script_name: String,
}

impl From<&super::TextDescriptionData> for TextDescriptionType {
    fn from(text_desc: &super::TextDescriptionData) -> Self {
        // Convert the raw bytes into a TextDescriptionToml
        TextDescriptionType::from_bytes(text_desc.0.as_slice())
            .expect("Failed to parse textDescriptionData")
    }
}

impl TextDescriptionType {
    /// Creates a new `TextDescription` with default values for non-provided fields.
    #[allow(unused)]
    fn new(description: &str) -> Self {
        Self {
            ascii: description.to_string(),
            unicode: description.to_string(),
            unicode_language_code: 0, // Undefined
            scriptcode_code: 0,       // Roman
            mac_script_name: String::new(),
        }
    }

    /// Attempts to parse a `textDescriptionData` tag from a byte buffer.
    fn from_bytes(buf: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(buf);

        // 1. Signature (4 bytes) & Reserved (4 bytes)
        let mut signature = [0u8; 4];
        cursor
            .read_exact(&mut signature)
            .map_err(|e| e.to_string())?;
        if &signature != b"desc" {
            return Err("Invalid TextDescription signature".to_string());
        }
        cursor
            .seek(SeekFrom::Current(4))
            .map_err(|e| e.to_string())?; // Skip reserved

        // 2. ASCII description
        let mut len_buf = [0u8; 4];
        cursor.read_exact(&mut len_buf).map_err(|e| e.to_string())?;
        let ascii_len = u32::from_be_bytes(len_buf) as usize;
        let mut ascii_bytes = vec![0u8; ascii_len];
        cursor
            .read_exact(&mut ascii_bytes)
            .map_err(|e| e.to_string())?;
        // The length includes the null terminator, so we trim it before converting.
        let ascii = str::from_utf8(&ascii_bytes[..ascii_len.saturating_sub(1)])
            .map_err(|e| e.to_string())?
            .to_string();

        // 3. Unicode language code
        cursor.read_exact(&mut len_buf).map_err(|e| e.to_string())?;
        let unicode_language_code = u32::from_be_bytes(len_buf);

        // 4. Unicode description
        cursor.read_exact(&mut len_buf).map_err(|e| e.to_string())?;
        let unicode_len = u32::from_be_bytes(len_buf) as usize; // Length in characters
        let mut unicode_bytes = vec![0u8; unicode_len * 2];
        cursor
            .read_exact(&mut unicode_bytes)
            .map_err(|e| e.to_string())?;
        let unicode_u16s: Vec<u16> = unicode_bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        let unicode = String::from_utf16(&unicode_u16s).map_err(|e| e.to_string())?;

        // 5. ScriptCode code
        let mut script_buf = [0u8; 2];
        cursor
            .read_exact(&mut script_buf)
            .map_err(|e| e.to_string())?;
        let scriptcode_code = u16::from_be_bytes(script_buf);

        // 6. Macintosh script name (padded to 67 bytes total)
        let mut mac_len_buf = [0u8; 1];
        cursor
            .read_exact(&mut mac_len_buf)
            .map_err(|e| e.to_string())?;
        let mac_len = mac_len_buf[0] as usize;
        if mac_len > 66 {
            return Err("Invalid Macintosh script name length".to_string());
        }
        let mut mac_bytes = vec![0u8; mac_len];
        cursor
            .read_exact(&mut mac_bytes)
            .map_err(|e| e.to_string())?;
        let mut mac_script_name = str::from_utf8(&mac_bytes)
            .map_err(|e| e.to_string())?
            .trim_end_matches('\0')
            .to_string();

        if mac_script_name == ascii {
            mac_script_name.clear(); // If the mac script name is the same as ASCII, clear it
        }
        Ok(TextDescriptionType {
            ascii,
            unicode,
            unicode_language_code,
            scriptcode_code,
            mac_script_name,
        })
    }
}

use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U32, U16};

use crate::tag::tagdata::TextDescriptionData;

#[repr(C, packed)]
#[derive(IntoBytes, Unaligned, KnownLayout, Immutable)]
pub struct WriteAsciiLayout {
    /// TagData signature, must be `b"desc"`.
    pub signature: [u8; 4],
    /// Reserved, must be 0.
    pub reserved: [u8; 4],
    /// ASCII description length (including null terminator).
    pub ascii_length: U32<BigEndian>,
} 

impl WriteAsciiLayout {
    /// Creates a new `WriteAsciiLayout` with the signature 'desc' and initializes
    /// the ASCII description length.
    pub fn new(ascii_length: u32) -> Self {
        Self {
            signature: *b"desc",
            reserved: [0; 4],
            ascii_length: U32::new(ascii_length),
        }
    }
}

#[repr(C, packed)]
#[derive(Default, IntoBytes, Unaligned, KnownLayout, Immutable)]
pub struct WriteUnicodeLayout {
    /// Unicode language code (4 bytes).
    /// Formed by a 2-byte ISO 639-1 language code
    /// and a 2-byte ISO 3166-1 country code (e.g., b"enUS").
    pub unicode_language_code: U32<BigEndian>,
    /// Unicode description length in 16-bit characters.
    pub unicode_length: U32<BigEndian>,
    // The UTF-16 encoded string data follows this struct.
    // The ICC specification requires this to be big-endian (UTF-16BE).
}


#[repr(C, packed)]
#[derive(FromBytes, IntoBytes, Unaligned, KnownLayout, Immutable)]
pub struct WriteMacScriptLayout {
    /// ScriptCode code for the Macintosh script name.
    pub scriptcode_code: U16<BigEndian>,
    /// Macintosh script name length (up to 66 bytes).
    pub mac_script_name_length: u8,
    /// Macintosh script name.
    pub mac_script_name: [u8; 67],
}

impl Default for WriteMacScriptLayout {
    fn default() -> Self {
        Self {
            scriptcode_code: U16::new(0),
            mac_script_name_length: 0,
            mac_script_name: [0; 67],
        }
    }
}

impl TextDescriptionData {
    pub fn set_ascii(&mut self, ascii: &str ) {
        let mut buf = Vec::new();
        let ascii_bytes = CString::new(ascii)
            .expect("ASCII string must be valid")
            .into_bytes_with_nul(); // Convert to bytes with null terminator
        let ascii_len = ascii_bytes.len();
        buf.extend_from_slice(WriteAsciiLayout::new(ascii_len as u32).as_bytes());
        buf.extend_from_slice(ascii_bytes.as_slice());
        buf.extend_from_slice(WriteUnicodeLayout::default().as_bytes());
        buf.extend_from_slice(WriteMacScriptLayout::default().as_bytes());
        self.0 = buf.to_vec();
    }

}
